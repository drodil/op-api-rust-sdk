use crate::options::Options;
use crate::requests::Requests;
use log::debug;
use serde::Deserialize;
use std::error::Error;

/// Describes a single Account
#[derive(Deserialize, Debug)]
pub struct Account {
    /// A surrogate identifier for the bank account.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Account name. This is a name assigned by the servicer and bears no significance to the
    /// user.
    pub name: String,
    /// Nickname of the account, if assigned by the owner of the account.
    pub nickname: Option<String>,
    /// Current balance of the account. Note: This field will only be returned after the end user
    /// has provided consent to the consuming application.
    pub balance: Option<f64>,
    /// Code of the currency of the account. All currency codes are ISO 4217-compliant strings.
    pub currency: String,
    /// "IBAN".
    #[serde(rename = "identifierScheme")]
    pub identifier_scheme: String,
    /// Account identifier. Follows the scheme described by the field identifierScheme.
    pub identifier: String,
    /// Identifier of the scheme used for identifying the servicer.
    #[serde(rename = "servicerScheme")]
    pub servicer_scheme: String,
    /// Identifier of the servicing bank. Follows the scheme described by the servicerScheme
    /// parameter.
    #[serde(rename = "servicerIdentifier")]
    pub servicer_identifier: String,
}

/// Describes a list of Accounts
#[derive(Deserialize, Debug)]
pub struct AccountList {
    pub accounts: Vec<Account>,
}

/// Accounts client.
///
/// This client is used to access the OP Accounts API.
pub struct Accounts {
    options: Options,
}

impl Accounts {
    /// Creates new Accounts client.
    ///
    /// Bear in mind that this API is implemented to follow v3 so you must
    /// specify v3 as version for the Options.
    pub fn new(options: Options) -> Accounts {
        Accounts { options }
    }

    /// Gets all accounts from the API and returns list of them.
    pub async fn accounts(&self) -> Result<AccountList, Box<dyn Error>> {
        let url = format!("/accounts/{}/accounts", self.options.version());
        let resp = Requests::get(&self.options, &url).await;
        match resp {
            Ok(response) => {
                debug!("Accounts response: {:#?}", response);
                let accounts: AccountList = response.json().await?;
                Ok(accounts)
            }
            Err(er) => Err(er.into()),
        }
    }

    /// Gets single account from the API based on accountId.
    pub async fn account(&self, account_id: String) -> Result<Account, Box<dyn Error>> {
        let url = format!(
            "/accounts/{}/accounts/{}",
            self.options.version(),
            account_id
        );
        let resp = Requests::get(&self.options, &url).await;
        match resp {
            Ok(response) => {
                debug!("Account response: {:#?}", response);
                let account: Account = response.json().await?;
                Ok(account)
            }
            Err(er) => Err(er.into()),
        }
    }
}
