//! One-time tickets for downloads
//!
//! A download the browser does itself (a link) can't send the
//! `session-token` header. So the webui first asks for a ticket for the
//! path it wants to download and adds it as `?download-ticket=` to the
//! url. The ticket stands in for the session once, only for that path and
//! only for a minute.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::FromRef;

use super::data::Token;
use crate::AppState;

const TICKET_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Debug)]
struct Ticket {
	session: Token,
	path: String,
	expires: Instant,
}

#[derive(Debug, Clone, Default)]
pub struct DownloadTickets {
	inner: Arc<Mutex<HashMap<String, Ticket>>>,
}

impl DownloadTickets {
	pub fn new() -> Self {
		Self::default()
	}

	/// Returns a new ticket for the given session and request path
	pub fn create(&self, session: Token, path: String) -> String {
		let ticket = Token::new().to_string();
		let mut tickets = self.inner.lock().unwrap();
		let now = Instant::now();
		tickets.retain(|_, t| t.expires > now);
		tickets.insert(
			ticket.clone(),
			Ticket {
				session,
				path,
				expires: now + TICKET_TIMEOUT,
			},
		);
		ticket
	}

	/// Uses up the ticket and returns its session token if the ticket is
	/// valid for this path
	pub fn take(&self, ticket: &str, path: &str) -> Option<Token> {
		let ticket = self.inner.lock().unwrap().remove(ticket)?;
		(ticket.path == path && ticket.expires > Instant::now())
			.then_some(ticket.session)
	}
}

impl FromRef<AppState> for DownloadTickets {
	fn from_ref(state: &AppState) -> Self {
		state.download_tickets.clone()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn ticket_is_single_use_and_bound_to_path() {
		let tickets = DownloadTickets::new();
		let session = Token::new();

		let t = tickets.create(session.clone(), "/api/a".into());
		assert!(tickets.take(&t, "/api/b").is_none());
		// a wrong path uses it up as well
		assert!(tickets.take(&t, "/api/a").is_none());

		let t = tickets.create(session.clone(), "/api/a".into());
		assert_eq!(
			tickets.take(&t, "/api/a").map(|s| s.to_string()),
			Some(session.to_string())
		);
		assert!(tickets.take(&t, "/api/a").is_none());
	}
}
