/// Options for requests to https://op-developer.fi
#[derive(Default)]
pub struct Options {
    api_key: String,
    authorization: String,
    version: String,
    base_url: String,
}

impl Options {
    /// Creates new Options struct for production access
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

    /// Creates new Options struct for sandbox access
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
    pub fn get_base_url(&self) -> &str {
        &self.base_url
    }

    /// Gets API key for requests.
    ///
    /// This is used as HTTP header x-api-key.
    pub fn get_api_key(&self) -> &str {
        &self.api_key
    }

    /// Gets Authorization for requests.
    ///
    /// This is used together with 'Bearer ' in the Authorization HTTP header.
    pub fn get_authorization(&self) -> &str {
        &self.authorization
    }

    /// Sets API version for requests.
    ///
    /// Some APIs are on different version (AccountsV3 for example).
    /// This is information is used to construct the request URL.
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Returns API version for requests.
    pub fn get_version(&self) -> &str {
        &self.version
    }
}
