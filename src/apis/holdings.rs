//! API implementation for
//! [HoldingsV1](https://op-developer.fi/docs/api/3VZiIRoT2EowKC6yeA0gqQ/Holdings)
//! API

use crate::model::holdings::HoldingsInformation;
use crate::options::Options;
use crate::requests::Requests;
use log::debug;
use std::error::Error;
use std::sync::Arc;

/// Holdings client.
///
/// This client is used to access the OP HoldingsV1 API.
pub struct HoldingsApi {
    options: Arc<Options>,
}

impl HoldingsApi {
    /// Creates new Holdings API.
    ///
    /// Bear in mind that this API is implemented to follow v1 so you must
    /// specify v1 as version for the Options.
    pub fn new(options: Arc<Options>) -> HoldingsApi {
        HoldingsApi { options }
    }

    /// Gets holdings information from the API.
    pub async fn holdings(&self) -> Result<HoldingsInformation, Box<dyn Error>> {
        let url = format!("/holdings/info/{}/holdings", self.options.version());
        let response = Requests::get(&self.options, &url, None::<()>).await?;
        debug!("Holdings response: {:#?}", response);
        let holdings: HoldingsInformation = response.json().await?;
        Ok(holdings)
    }
}
