//! API implementation for
//! [AccountsV3](https://op-developer.fi/docs/api/3Oo5zCujXGw2SGEi00skug/OP%20Accounts%20V3.0%20API)
//! API

use crate::model::accounts::*;
use crate::options::Options;
use crate::requests::Requests;
use log::debug;
use std::error::Error;
use std::sync::Arc;

/// Accounts client.
///
/// This client is used to access the OP AccountsV3 API.
pub struct AccountsApi {
    options: Arc<Options>,
}

impl AccountsApi {
    /// Creates new Accounts API.
    ///
    /// Bear in mind that this API is implemented to follow v3 so you must
    /// specify v3 as version for the Options.
    pub fn new(options: Arc<Options>) -> AccountsApi {
        AccountsApi { options }
    }

    /// Gets all accounts from the API and returns list of them.
    pub async fn accounts(&self) -> Result<AccountList, Box<dyn Error>> {
        let url = format!("/accounts/{}/accounts", self.options.version());
        let response = Requests::get(&self.options, &url, None::<()>).await?;
        debug!("Accounts response: {:#?}", response);
        let accounts: AccountList = response.json().await?;
        Ok(accounts)
    }

    /// Gets single account from the API based on accountId.
    pub async fn account(&self, account_id: String) -> Result<Account, Box<dyn Error>> {
        let url = format!(
            "/accounts/{}/accounts/{}",
            self.options.version(),
            account_id
        );
        let response = Requests::get(&self.options, &url, None::<()>).await?;
        debug!("Account response: {:#?}", response);
        let account: Account = response.json().await?;
        Ok(account)
    }

    /// Gets all transactions for a single account with account id
    /// with optional parameters for filtering the results.
    pub async fn transactions(
        &self,
        account_id: String,
        params: Option<TransactionParams>,
    ) -> Result<TransactionList, Box<dyn Error>> {
        let url = format!(
            "/accounts/{}/accounts/{}/transactions",
            self.options.version(),
            account_id
        );
        let response = Requests::get(&self.options, &url, params).await?;
        debug!("Transactions response: {:#?}", response);
        let list: TransactionList = response.json().await?;
        Ok(list)
    }
}
