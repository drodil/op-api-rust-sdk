use crate::options::Options;
use log::debug;
use reqwest::{Client, Error, RequestBuilder, Response};
use serde::Serialize;

pub struct Requests;

/// Constructs string URL from base url and API url.
fn get_request_url(options: &Options, url: &str) -> String {
    format!("{base_url}{url}", base_url = options.base_url(), url = url)
}

/// Sets necessary headers for the request.
fn set_headers(options: &Options, builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-api-key", options.api_key())
        .header(
            "Authorization",
            format!("{} {}", "Bearer", options.authorization()),
        )
        .header("Accept", "application/json")
}

/// Sets query parameters for the request.
fn set_query_params<T: Serialize>(query: Option<T>, builder: RequestBuilder) -> RequestBuilder {
    match query {
        Some(q) => builder.query(&q),
        None => builder,
    }
}

/// Internal requests functionality to ease client development.
///
/// These functions set up all necessary headers and run the request
/// asynchronously.
impl Requests {
    /// Performs GET request to API specified with url.
    pub async fn get<T: Serialize>(
        options: &Options,
        url: &str,
        query: Option<T>,
    ) -> Result<Response, Error> {
        let request_url = get_request_url(options, url);
        let builder = Client::new().get(&request_url);
        let client = set_headers(options, set_query_params(query, builder));
        debug!("Sending request: {:?}", client);
        client.send().await
    }
}
