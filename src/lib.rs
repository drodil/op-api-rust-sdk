//! Provides clients and data structures to work with
//! [OP API](https://op-developer.fi)
//!
//! # Clients
//!
//! All available clients can be found from the *apis* module.
//! Clients require specific options to send requests which
//! are defined in the options module.
//!
//! # Options
//!
//! All calls to any of the APIs require API key which can be
//! requested from the OP-Developer portal. For production access
//! the requests additionally need an OAuth2 token for authorization
//! of the user.

pub mod apis;
pub mod options;
pub use apis::*;
mod requests;
