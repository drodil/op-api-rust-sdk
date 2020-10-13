//! Provides clients and data structures to work with
//! [OP API](https://op-developer.fi)
//!
//! # Client
//!
//! All available API functions can be found from the *client* module.
//! Client requires specific options to send requests which
//! are defined in the options module.
//!
//! # Options
//!
//! All calls to any of the APIs require API key which can be
//! requested from the OP-Developer portal. For production access
//! the requests additionally need an OAuth2 token for authorization
//! of the user.
//!
//! # Model
//!
//! The model contains all necessary structures for REST communication.
//! Each endpoint has it's own module.

pub mod model;
pub mod options;
pub use model::*;
pub mod client;

mod apis;
mod requests;
