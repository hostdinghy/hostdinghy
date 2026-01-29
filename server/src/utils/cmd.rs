use std::{
	io,
	path::Path,
	pin::Pin,
	process::Stdio,
	task::{self, Poll},
};

use simple_bytes::{BytesOwned, BytesRead, BytesSeek, BytesWrite};
use tokio::{
	io::{AsyncRead, ReadBuf},
	process::{Child, ChildStderr, ChildStdout, Command},
};

use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum CmdError {
	#[error("Failed to run command: {command}, message: {message}")]
	Command { command: String, message: String },
}

impl CmdError {
	pub fn cmd(command: impl ToString, message: impl ToString) -> Self {
		Self::Command {
			command: command.to_string(),
			message: message.to_string(),
		}
	}
}

pub fn cmd(args: &[&str]) -> CmdBuilder {
	assert!(!args.is_empty(), "args should not be empty");
	let mut c = Command::new(args[0]);
	c.args(&args[1..]);

	CmdBuilder {
		display: args.join(" "),
		inner: c,
	}
}

#[derive(Debug)]
pub struct CmdBuilder {
	display: String,
	inner: Command,
}

impl CmdBuilder {
	#[allow(dead_code)]
	pub fn arg(mut self, arg: &str) -> Self {
		self.inner.arg(arg);
		self
	}

	pub fn arg_opt(mut self, arg: Option<&str>) -> Self {
		if let Some(a) = arg {
			self.inner.arg(a);
		}
		self
	}

	#[allow(dead_code)]
	pub fn current_dir(mut self, path: impl AsRef<Path>) -> Self {
		self.inner.current_dir(path);
		self
	}

	pub fn as_root(mut self) -> Self {
		#[cfg(unix)]
		self.inner.uid(0);
		self
	}

	pub async fn run(mut self) -> Result<String, CmdError> {
		let output = self
			.inner
			.output()
			.await
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		if !output.status.success() {
			return Err(CmdError::cmd(
				self.display,
				String::from_utf8_lossy(&output.stderr),
			));
		}

		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}

	pub async fn spawn_readable_stdout(
		mut self,
	) -> Result<ChildReadableStdout, CmdError> {
		let mut child = self
			.inner
			.stdin(Stdio::null())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		Ok(ChildReadableStdout {
			stdout: child.stdout.take().unwrap(),
			stderr: child.stderr.take().unwrap(),
			child,
			stderr_buf: BytesOwned::new(),
		})
	}
}

#[derive(Debug)]
pub struct ChildReadableStdout {
	child: Child,
	stdout: ChildStdout,
	stderr: ChildStderr,
	stderr_buf: BytesOwned,
}

const BUF_LEN: usize = 1024;

impl ChildReadableStdout {
	fn read_stderr(
		&mut self,
		cx: &mut task::Context<'_>,
	) -> Poll<io::Result<String>> {
		// we need to read the stderr and then output it as error
		if self.stderr_buf.remaining().len() < BUF_LEN {
			let buf_len = self.stderr_buf.len();
			self.stderr_buf.resize(buf_len + BUF_LEN);
		}

		let mut read_buf = ReadBuf::new(self.stderr_buf.remaining_mut());

		match Pin::new(&mut self.stderr).poll_read(cx, &mut read_buf) {
			Poll::Ready(Ok(())) => {
				let read = read_buf.filled().len();
				self.stderr_buf.advance(read);

				if read > 0 {
					return Poll::Pending;
				}

				// read the entire stderr now output
				let err_msg = String::from_utf8_lossy(
					&self.stderr_buf.as_slice()[..self.stderr_buf.position()],
				);
				Poll::Ready(Ok(err_msg.into_owned()))
			}
			Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
			Poll::Pending => Poll::Pending,
		}
	}
}

impl AsyncRead for ChildReadableStdout {
	fn poll_read(
		mut self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
		buf: &mut ReadBuf<'_>,
	) -> Poll<io::Result<()>> {
		match self.child.try_wait() {
			// if the status was not success we need to read stderr
			Ok(Some(status)) if !status.success() => {
				return match self.read_stderr(cx) {
					Poll::Ready(Ok(err_msg)) => {
						Poll::Ready(Err(io::Error::new(
							io::ErrorKind::Other,
							format!(
								"command failed with status {status}: {err_msg}"
							),
						)))
					}
					Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
					Poll::Pending => Poll::Pending,
				};
			}
			Err(e) => return Poll::Ready(Err(e)),
			_ => {}
		}

		Pin::new(&mut self.stdout).poll_read(cx, buf)
	}
}

impl Drop for ChildReadableStdout {
	fn drop(&mut self) {
		if let Err(e) = self.child.start_kill() {
			error!("Failed to kill process: {e}");
		}
	}
}
