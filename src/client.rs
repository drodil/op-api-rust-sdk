//! This module contains Client to communicate with different APIs
//! defined in [OP API](https://op-developer.fi/docs).

#[cfg(feature = "accounts")]
use crate::apis::accounts::AccountsApi;
#[cfg(feature = "accounts")]
use crate::model::accounts::*;
use crate::options::Options;
use std::error::Error;
use std::sync::Arc;

pub struct Client {
    options: Arc<Options>,
    #[cfg(feature = "accounts")]
    accounts_api: AccountsApi,
}

impl Client {
    /// Creates new client for all APIs in op-developer.fi.
    ///
    /// Options must be a reference counting pointer to Options
    /// structure (std::sync::Arc).
    pub fn new(options: Arc<Options>) -> Client {
        Client {
            options: options.clone(),
            #[cfg(feature = "accounts")]
            accounts_api: AccountsApi::new(options),
        }
    }

    /// Returns options used by this client.
    ///
    /// Note that the return value is a clone of the options inside
    /// reference counting pointer to the options (Arc). This is due
    /// to requirement that the user of this client can change the
    /// parameters and they are reflected to all future requests done
    /// by the client.
    pub fn options(&self) -> Arc<Options> {
        self.options.clone()
    }

    /// Gets all accounts from the API and returns list of them.
    #[cfg(feature = "accounts")]
    pub async fn accounts(&self) -> Result<AccountList, Box<dyn Error>> {
        self.accounts_api.accounts().await
    }

    /// Gets single account from the API based on accountId.
    #[cfg(feature = "accounts")]
    pub async fn account(&self, account_id: String) -> Result<Account, Box<dyn Error>> {
        self.accounts_api.account(account_id).await
    }

    /// Gets all transactions for a single account with account id
    /// with optional parameters for filtering the results.
    #[cfg(feature = "accounts")]
    pub async fn transactions(
        &self,
        account_id: String,
        params: Option<TransactionParams>,
    ) -> Result<TransactionList, Box<dyn Error>> {
        self.accounts_api.transactions(account_id, params).await
    }
}
