use crate::options::Options;
use log::debug;
use reqwest::{Client, Error, RequestBuilder, Response};

pub struct Requests;

/// Constructs string URL from base url and API url.
fn get_request_url(options: &Options, url: &str) -> String {
    format!("{base_url}{url}", base_url = options.base_url(), url = url)
}

/// Sets necessary headers for requests.
fn set_headers(options: &Options, builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-api-key", options.api_key())
        .header(
            "Authorization",
            format!("{} {}", "Bearer", options.authorization()),
        )
        .header("Accept", "application/json")
}

/// Internal requests functionality to ease client development.
///
/// These functions set up all necessary headers and run the request
/// asynchronously.
impl Requests {
    /// Performs GET request to API specified with url.
    pub async fn get(options: &Options, url: &str) -> Result<Response, Error> {
        let request_url = get_request_url(options, url);
        debug!("Sending GET request to {}", request_url);
        let builder = Client::new().get(&request_url);
        let client = set_headers(options, builder);
        client.send().await
    }
}
