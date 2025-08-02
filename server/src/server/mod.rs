use std::{path::Path, sync::Arc};

use api::error::{Error, WithMessage as _};
use axum::extract::Request;
use hyper::body::Incoming;
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::{net::TcpListener, task};
use tokio_rustls::{
	TlsAcceptor,
	rustls::{
		ServerConfig,
		pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject as _},
	},
};

use tower::Service as _;
use tracing::{error, warn};

use crate::utils::{hostdinghy_dir, verify_root};

mod cert;
pub mod config;
pub mod error;
pub mod router;
mod utils;

pub use cert::{maybe_create_cert, read_cert};
pub use config::Config;
pub use utils::Authenticated;

pub async fn serve() {
	let res = inner_serve().await;

	if let Err(e) = res {
		error!("Serve command failed: {e}");
	}
}

// todo maybe this should return a cli error
async fn inner_serve() -> Result<(), Error> {
	verify_root().await?;
	let hostdinghy_dir = hostdinghy_dir()?;
	let mut cfg = Config::read(&hostdinghy_dir).await?;

	if cfg.domain.is_empty() {
		return Err(Error::any(
			"run setup server to define a domain",
			"domain required to run the internal server",
		));
	}

	maybe_create_cert(&mut cfg, &hostdinghy_dir).await?;

	// this is not really necessary since we don't do anything async but
	// since everything else is async, might be good practice
	let n_hostdinghy_dir = hostdinghy_dir.clone();
	let rustls_config =
		task::spawn_blocking(move || rustls_server_config(n_hostdinghy_dir))
			.await
			.unwrap()?;

	let tls_acceptor = TlsAcceptor::from(Arc::new(rustls_config));
	let tcp_listener = TcpListener::bind("[::]:4242")
		.await
		.with_message("failed to bind to [::]:4242")?;

	let app = router::app(&hostdinghy_dir, cfg).await?;

	loop {
		let tower_service = app.clone();
		let tls_acceptor = tls_acceptor.clone();

		// Wait for new tcp connection
		let (cnx, addr) = tcp_listener
			.accept()
			.await
			.with_message("failed to accept tcp connection")?;

		tokio::spawn(async move {
			// Wait for tls handshake to happen
			let Ok(stream) = tls_acceptor.accept(cnx).await else {
				error!("error during tls handshake connection from {}", addr);
				return;
			};

			let stream = TokioIo::new(stream);
			let hyper_service = hyper::service::service_fn(
				move |request: Request<Incoming>| {
					// We don't need to call `poll_ready` since `Router` is always ready.
					tower_service.clone().call(request)
				},
			);

			let ret = hyper_util::server::conn::auto::Builder::new(
				TokioExecutor::new(),
			)
			.serve_connection_with_upgrades(stream, hyper_service)
			.await;

			if let Err(err) = ret {
				warn!("error serving connection from {}: {}", addr, err);
			}
		});
	}
}

fn rustls_server_config(
	hostdinghy_dir: impl AsRef<Path>,
) -> Result<ServerConfig, Error> {
	let key = cert::key_path(&hostdinghy_dir);
	let cert = cert::cert_path(&hostdinghy_dir);

	let key = PrivateKeyDer::from_pem_file(key)
		.with_message("failed to read private key")?;

	let certs = CertificateDer::pem_file_iter(cert)
		.with_message("failed to read certificate")?
		.filter_map(|cert| match cert {
			Ok(c) => Some(c),
			Err(e) => {
				error!("failed to parse certificate: {e}");
				None
			}
		})
		.collect();

	let mut config = ServerConfig::builder()
		.with_no_client_auth()
		.with_single_cert(certs, key)
		.with_message("failed to create server config")?;

	config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

	Ok(config)
}
