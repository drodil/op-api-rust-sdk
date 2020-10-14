//! API implementation for
//! [Funds data V1](https://op-developer.fi/docs/api/5lWcjqy3JY2G2y4UGmS6Yw/Funds-data#operation/getFunds)
//! API

use crate::model::funds::*;
use crate::options::Options;
use crate::requests::Requests;
use log::debug;
use std::error::Error;
use std::sync::Arc;

/// Funds client.
///
/// This client is used to access the OP FundsV1 API.
pub struct FundsApi {
    options: Arc<Options>,
}

impl FundsApi {
    /// Creates new Funds API.
    ///
    /// Bear in mind that this API is implemented to follow v1 so you must
    /// specify v1 as version for the Options.
    pub fn new(options: Arc<Options>) -> FundsApi {
        FundsApi { options }
    }

    /// Gets all funds from the API and returns them.
    pub async fn funds(&self) -> Result<Funds, Box<dyn Error>> {
        let url = format!("/{}/funds", self.options.version());
        let response = Requests::get(&self.options, &url, None::<()>).await?;
        debug!("Funds response: {:#?}", response);
        let funds: Funds = response.json().await?;
        Ok(funds)
    }
}
