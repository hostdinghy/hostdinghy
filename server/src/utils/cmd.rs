use std::{
	future::poll_fn,
	io,
	path::Path,
	pin::{Pin, pin},
	process::Stdio,
	task::{self, Poll},
};

use simple_bytes::{BytesOwned, BytesRead, BytesSeek, BytesWrite};
use tokio::{
	io::{AsyncRead, AsyncWrite, ReadBuf},
	process::{Child, ChildStderr, ChildStdin, ChildStdout, Command},
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
			display: self.display,
			stdout: child.stdout.take().unwrap(),
			stderr: StderrReader::new(child.stderr.take().unwrap()),
			child,
			poll_child_exit: false,
		})
	}

	pub async fn spawn_writable_stdin(
		mut self,
	) -> Result<ChildWritableStdin, CmdError> {
		let mut child = self
			.inner
			.stdin(Stdio::piped())
			.stdout(Stdio::null())
			.stderr(Stdio::piped())
			.spawn()
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		Ok(ChildWritableStdin {
			display: self.display,
			stdin: child.stdin.take().unwrap(),
			stderr: StderrReader::new(child.stderr.take().unwrap()),
			child,
		})
	}
}

#[derive(Debug)]
pub struct StderrReader {
	stderr: ChildStderr,
	stderr_buf: BytesOwned,
}

impl StderrReader {
	fn new(stderr: ChildStderr) -> Self {
		Self {
			stderr,
			stderr_buf: BytesOwned::new(),
		}
	}

	async fn read(&mut self) -> io::Result<String> {
		poll_fn(|cx| self.read_stderr(cx)).await
	}

	fn read_stderr(
		&mut self,
		cx: &mut task::Context<'_>,
	) -> Poll<io::Result<String>> {
		const BUF_LEN: usize = 1024;

		// we need to read the stderr and then output it as error
		if self.stderr_buf.remaining().len() < BUF_LEN / 2 {
			let buf_len = self.stderr_buf.len();
			self.stderr_buf.resize(buf_len + BUF_LEN);
		}

		// read until EOF
		loop {
			let mut read_buf = ReadBuf::new(self.stderr_buf.remaining_mut());

			match Pin::new(&mut self.stderr).poll_read(cx, &mut read_buf) {
				Poll::Ready(Ok(())) => {
					let read = read_buf.filled().len();
					self.stderr_buf.advance(read);

					if read > 0 {
						continue;
					}

					// read the entire stderr now output
					let err_msg = String::from_utf8_lossy(
						&self.stderr_buf.as_slice()
							[..self.stderr_buf.position()],
					);
					break Poll::Ready(Ok(err_msg.into_owned()));
				}
				Poll::Ready(Err(e)) => break Poll::Ready(Err(e)),
				Poll::Pending => break Poll::Pending,
			}
		}
	}
}

#[derive(Debug)]
pub struct ChildReadableStdout {
	display: String,
	child: Child,
	stdout: ChildStdout,
	stderr: StderrReader,
	poll_child_exit: bool,
}

impl ChildReadableStdout {
	pub fn exited_with_error(&mut self) -> bool {
		match self.child.try_wait() {
			Ok(Some(status)) if !status.success() => true,
			Err(_) => true,
			_ => false,
		}
	}

	pub async fn wait(mut self) -> Result<(), CmdError> {
		let status = self
			.child
			.wait()
			.await
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		if !status.success() {
			let err =
				self.stderr.read().await.unwrap_or_else(|e| e.to_string());
			return Err(CmdError::cmd(self.display.clone(), err));
		}

		Ok(())
	}

	pub fn wait_for_child_exit(&mut self, poll: bool) {
		self.poll_child_exit = poll;
	}

	fn poll_child_error(
		&mut self,
		cx: &mut task::Context<'_>,
	) -> Option<Poll<io::Error>> {
		match self.child.try_wait() {
			// if the status was not success we need to read stderr
			Ok(Some(status)) if !status.success() => {
				return Some(match self.stderr.read_stderr(cx) {
					Poll::Ready(Ok(err_msg)) => Poll::Ready(io::Error::new(
						io::ErrorKind::Other,
						format!(
							"command failed with status {status}: {err_msg}"
						),
					)),
					Poll::Ready(Err(e)) => Poll::Ready(e),
					Poll::Pending => Poll::Pending,
				});
			}
			Err(e) => return Some(Poll::Ready(e)),
			_ => {}
		}

		None
	}
}

impl AsyncRead for ChildReadableStdout {
	fn poll_read(
		mut self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
		buf: &mut ReadBuf<'_>,
	) -> Poll<io::Result<()>> {
		// loop to wait until child has exited
		loop {
			if let Some(err) = self.poll_child_error(cx) {
				return err.map(|e| Err(e));
			}

			match Pin::new(&mut self.stdout).poll_read(cx, buf) {
				Poll::Ready(Ok(())) => {
					let read = buf.filled().len();

					if read > 0 {
						return Poll::Ready(Ok(()));
					}
				}
				Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
				Poll::Pending => return Poll::Pending,
			}

			if !self.poll_child_exit {
				return Poll::Ready(Ok(()));
			}

			// read is completed
			let wait_fut = self.child.wait();
			let wait_fut = pin!(wait_fut);

			match wait_fut.poll(cx) {
				// if the exit status was success we are finished
				Poll::Ready(Ok(status)) if status.success() => {
					return Poll::Ready(Ok(()));
				}
				// if the status was not success we ignore what
				// gets returned since poll_try_wait will handle the
				// error case
				Poll::Ready(_) => continue,
				Poll::Pending => return Poll::Pending,
			}
		}
	}
}

impl Drop for ChildReadableStdout {
	fn drop(&mut self) {
		if let Err(e) = self.child.start_kill() {
			error!("Failed to kill process: {e}");
		}
	}
}

#[derive(Debug)]
pub struct ChildWritableStdin {
	display: String,
	child: Child,
	stdin: ChildStdin,
	stderr: StderrReader,
}

impl ChildWritableStdin {
	pub async fn wait(mut self) -> Result<(), CmdError> {
		let status = self
			.child
			.wait()
			.await
			.map_err(|e| CmdError::cmd(&self.display, e))?;

		eprintln!("child exited with status: {status}");

		if !status.success() {
			let err =
				self.stderr.read().await.unwrap_or_else(|e| e.to_string());
			return Err(CmdError::cmd(self.display.clone(), err));
		}

		Ok(())
	}
}

impl AsyncWrite for ChildWritableStdin {
	fn poll_write(
		mut self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
		buf: &[u8],
	) -> Poll<io::Result<usize>> {
		match self.child.try_wait() {
			// if the status was not success we need to read stderr
			Ok(Some(status)) if !status.success() => {
				return match self.stderr.read_stderr(cx) {
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

		Pin::new(&mut self.stdin).poll_write(cx, buf)
	}

	fn poll_flush(
		mut self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
	) -> Poll<io::Result<()>> {
		Pin::new(&mut self.stdin).poll_flush(cx)
	}

	fn poll_shutdown(
		mut self: Pin<&mut Self>,
		cx: &mut task::Context<'_>,
	) -> Poll<io::Result<()>> {
		Pin::new(&mut self.stdin).poll_shutdown(cx)
	}
}

impl Drop for ChildWritableStdin {
	fn drop(&mut self) {
		if let Err(e) = self.child.start_kill() {
			error!("Failed to kill process: {e}");
		}
	}
}
