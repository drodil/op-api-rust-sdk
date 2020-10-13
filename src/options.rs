//! This module contains Options for the clients needed
//! to make requests to [OP API](https://op-developer.fi).

use std::sync::{Arc, RwLock};

/// Inner options values inside the RwLock.
#[derive(Default, Clone)]
struct OptionsInner {
    api_key: String,
    authorization: String,
    version: String,
    base_url: String,
}

/// Struct containing needed options for API clients.
#[derive(Default)]
pub struct Options {
    inner: RwLock<OptionsInner>,
}

impl Options {
    /// Creates new Options struct for production access.
    ///
    /// Keep in mind that authorization must be feched from the
    /// OAuth and passed here without 'Bearer' included.
    pub fn new(api_key: String, authorization: String) -> Arc<Options> {
        Arc::new(Options {
            inner: RwLock::new(OptionsInner {
                api_key,
                authorization,
                version: String::from("v1"),
                base_url: String::from("https://prod.apis.op-palvelut.fi/"),
            }),
        })
    }

    /// Creates new Options struct for sandbox access.
    ///
    /// Authorization for this is not required as it uses one of the
    /// predefined authorization keys in https://op-developer.fi/docs
    pub fn new_dev(api_key: String) -> Arc<Options> {
        Arc::new(Options {
            inner: RwLock::new(OptionsInner {
                api_key,
                authorization: String::from("b6910384440ce06f495976f96a162e2ab1bafbb4"),
                version: String::from("v1"),
                base_url: String::from("https://sandbox.apis.op-palvelut.fi/"),
            }),
        })
    }

    /// Gets base URL for requests.
    ///
    /// This is different for production and sandbox environments.
    pub fn base_url(&self) -> String {
        self.inner.read().unwrap().base_url.clone()
    }

    /// Sets base URL for all API requests.
    ///
    /// This is useful when testing the clients and can be used
    /// together with local test or mock server.
    pub fn set_base_url(&self, base_url: String) {
        self.inner.write().unwrap().base_url = base_url;
    }

    /// Gets API key for requests.
    ///
    /// This is used as HTTP header x-api-key.
    pub fn api_key(&self) -> String {
        self.inner.read().unwrap().api_key.clone()
    }

    /// Sets API key for API requests using these options.
    pub fn set_api_key(&self, api_key: String) {
        self.inner.write().unwrap().api_key = api_key;
    }

    /// Gets Authorization for requests.
    ///
    /// This is used together with 'Bearer ' in the Authorization HTTP header.
    pub fn authorization(&self) -> String {
        self.inner.read().unwrap().authorization.clone()
    }

    /// Sets authorization header value for API requests using these options.
    ///
    /// The authorization should not contain 'Bearer ' in front of the
    /// authorization key as it will be automatically preprended by the
    /// clients.
    pub fn set_authorization(&self, authorization: String) {
        self.inner.write().unwrap().authorization = authorization;
    }

    /// Sets API version for requests.
    ///
    /// Some APIs are on different version (AccountsV3 for example).
    /// This is information is used to construct the request URL.
    pub fn set_version(&self, version: String) {
        self.inner.write().unwrap().version = version;
    }

    /// Returns API version for requests.
    pub fn version(&self) -> String {
        self.inner.read().unwrap().version.clone()
    }
}
