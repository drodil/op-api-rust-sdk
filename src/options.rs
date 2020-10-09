/// Options for API clients.
#[derive(Default, Clone)]
pub struct Options {
    api_key: String,
    authorization: String,
    version: String,
    base_url: String,
}

impl Options {
    /// Creates new Options struct for production access.
    ///
    /// Keep in mind that authorization must be feched from the
    /// OAuth and passed here without 'Bearer' included.
    pub fn new(api_key: String, authorization: String) -> Options {
        Options {
            api_key,
            authorization,
            version: String::from("v1"),
            base_url: String::from("https://prod.apis.op-palvelut.fi/"),
        }
    }

    /// Creates new Options struct for sandbox access.
    ///
    /// Authorization for this is not required as it uses one of the
    /// predefined authorization keys in https://op-developer.fi/docs
    pub fn new_dev(api_key: String) -> Options {
        Options {
            api_key,
            authorization: String::from("b6910384440ce06f495976f96a162e2ab1bafbb4"),
            version: String::from("v1"),
            base_url: String::from("https://sandbox.apis.op-palvelut.fi/"),
        }
    }

    /// Gets base URL for requests.
    ///
    /// This is different for production and sandbox environments.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Sets base URL for all API requests.
    ///
    /// This is useful when testing the clients and can be used
    /// together with local test or mock server.
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = base_url;
        self
    }

    /// Gets API key for requests.
    ///
    /// This is used as HTTP header x-api-key.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Sets API key for API requests using these options.
    pub fn with_api_key(mut self, api_key: String) -> Self {
        self.api_key = api_key;
        self
    }

    /// Gets Authorization for requests.
    ///
    /// This is used together with 'Bearer ' in the Authorization HTTP header.
    pub fn authorization(&self) -> &str {
        &self.authorization
    }

    /// Sets authorization header value for API requests using these options.
    ///
    /// The authorization should not contain 'Bearer ' in front of the
    /// authorization key as it will be automatically preprended by the
    /// clients.
    pub fn with_authorization(mut self, authorization: String) -> Self {
        self.authorization = authorization;
        self
    }

    /// Sets API version for requests.
    ///
    /// Some APIs are on different version (AccountsV3 for example).
    /// This is information is used to construct the request URL.
    pub fn with_version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// Returns API version for requests.
    pub fn version(&self) -> &str {
        &self.version
    }
}
