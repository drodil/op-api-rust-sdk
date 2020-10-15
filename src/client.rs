//! This module contains Client to communicate with different APIs
//! defined in [OP API](https://op-developer.fi/docs).

use crate::apis::accounts::AccountsApi;
use crate::apis::funds::FundsApi;
use crate::apis::holdings::HoldingsApi;
use crate::model::accounts::*;
use crate::model::funds::*;
use crate::model::holdings::HoldingsInformation;
use crate::options::Options;
use std::error::Error;
use std::sync::Arc;

pub struct Client {
    options: Arc<Options>,
    accounts_api: AccountsApi,
    funds_api: FundsApi,
    holdings_api: HoldingsApi,
}

impl Client {
    /// Creates new client for all APIs in op-developer.fi.
    ///
    /// Options must be a reference counting pointer to Options
    /// structure (std::sync::Arc).
    pub fn new(options: Arc<Options>) -> Client {
        Client {
            options: options.clone(),
            accounts_api: AccountsApi::new(options.clone()),
            funds_api: FundsApi::new(options.clone()),
            holdings_api: HoldingsApi::new(options.clone()),
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
    pub async fn accounts(&self) -> Result<AccountList, Box<dyn Error>> {
        self.accounts_api.accounts().await
    }

    /// Gets single account from the API based on accountId.
    pub async fn account(&self, account_id: String) -> Result<Account, Box<dyn Error>> {
        self.accounts_api.account(account_id).await
    }

    /// Gets all funds from the API and returns of them.
    pub async fn funds(&self) -> Result<Funds, Box<dyn Error>> {
        self.funds_api.funds().await
    }

    /// Gets holdings information from the API and returns it.
    pub async fn holdings(&self) -> Result<HoldingsInformation, Box<dyn Error>> {
        self.holdings_api.holdings().await
    }

    /// Gets all transactions for a single account with account id
    /// with optional parameters for filtering the results.
    pub async fn transactions(
        &self,
        account_id: String,
        params: Option<TransactionParams>,
    ) -> Result<TransactionList, Box<dyn Error>> {
        self.accounts_api.transactions(account_id, params).await
    }
}
