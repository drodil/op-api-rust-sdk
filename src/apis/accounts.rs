use crate::options::Options;
use crate::requests::Requests;
use chrono::{DateTime, Utc};
use log::debug;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// Link inside the results of Accounts API.
#[derive(Deserialize, Debug)]
pub struct Link {
    pub href: String,
}

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

/// Optional parameters to fetch transactions.
#[derive(Serialize, Debug)]
pub struct TransactionParams {
    /// ISO 8601-compatible date-time string representing the earliest date-time from which
    /// transactions will be queried. Timezone must not be set. Set time to to 00:00:00 for
    /// Date-only queries.
    pub from_booking_datetime: Option<DateTime<Utc>>,
    /// ISO 8601-compatible date-time string representing the latest date-time up to which
    /// transactions will be queried. Timezone must not be set. Set time to 00:00:00 for Date-only
    /// queries.
    pub to_booking_datetime: Option<DateTime<Utc>>,
    /// Number of transactions to be returned per each page.
    pub page_size: Option<u32>,
    /// Paging token used to retrieve the next page of data. Tokens are available in the links
    /// located in the _links object.
    pub forward_paging_token: Option<String>,
}

/// Describes a single party in Transaction.
#[derive(Deserialize, Debug)]
pub struct TransactionParty {
    /// Account identifier schema.
    #[serde(rename = "accountIdentifierType")]
    pub account_identifier_type: String,
    /// Name of the account.
    #[serde(rename = "accountName")]
    pub account_name: String,
    /// Identifier of the party's bank account.
    #[serde(rename = "accountIdentifier")]
    pub account_identifier: String,
    /// A code identifying the party's servicer.
    #[serde(rename = "servicerIdentifier")]
    pub servicer_identifier: String,
    /// Type of the servicerIdentifier, i.e. the scheme of the servicer ID.
    #[serde(rename = "servicerIdentifierType")]
    pub servicer_identifier_type: String,
}

/// Describes a single Transaction for Account.
#[derive(Deserialize, Debug)]
pub struct Transaction {
    /// Surrogate identifier for the transaction.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Surrogate identifier for the account.
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// Archive ID of the transaction.
    #[serde(rename = "archiveId")]
    pub archive_id: Option<String>,
    /// Reference number used in the transaction.
    pub reference: Option<String>,
    /// A message sent with the transaction. Created by the payer and comprises free-form text.
    pub message: Option<String>,
    /// Amount transferred in the transaction. The value is a string decimal. Debit transactions
    /// are marked with a minus sign.
    pub amount: String,
    /// Currency of the transaction.
    pub currency: String,
    /// Enum(debit, credit)
    /// Describes whether the transaction is a debit of credit transaction.
    #[serde(rename = "creditDebitIndicator")]
    pub credit_debit_indicator: String,
    /// Balance of the account after the transaction.
    #[serde(rename = "accountBalance")]
    pub account_balance: String,
    /// Account information of the creditor. The response body will only contain this field if the
    /// transaction is of type debit, i.e. the counterparty in the transaction is the creditor.
    pub creditor: Option<TransactionParty>,
    /// Account information of the debtor. The response body will only contain this field if the
    /// transaction is of type credit, i.e. the counterparty in the transaction is the debtor.
    pub debtor: Option<TransactionParty>,
    /// Date and time the transaction was entered into book-keeping. ISO 8601-formatted date-time string.
    #[serde(rename = "bookingDateTime")]
    pub booking_datetime: DateTime<Utc>,
    /// The date and time when amount of the transaction was counted towards the balance of the
    /// account. ISO 8601-formatted date-time string.
    #[serde(rename = "valueDateTime")]
    pub value_datetaime: DateTime<Utc>,
    /// Enum(Authorised, AwaitingAuthorisation, Rejected, Revoked)
    /// Current status of the transaction.
    pub status: Option<String>,
    /// ISO 20022-compliant transaction code for the transaction.
    #[serde(rename = "isoTransactionCode")]
    pub iso_transaction_code: Option<String>,
    /// OP-specific transaction code for the transaction.
    #[serde(rename = "opTransactionCode")]
    pub op_transaction_code: Option<String>,
}

/// Describes links in the Transactions object.
#[derive(Deserialize, Debug)]
pub struct TransactionListLinks {
    pub next: Option<Link>,
}

/// Describes a list of Transactions.
#[derive(Deserialize, Debug)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
    #[serde(rename = "_links")]
    pub links: TransactionListLinks,
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
        let resp = Requests::get(&self.options, &url, None::<()>).await;
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
        let resp = Requests::get(&self.options, &url, None::<()>).await;
        match resp {
            Ok(response) => {
                debug!("Account response: {:#?}", response);
                let account: Account = response.json().await?;
                Ok(account)
            }
            Err(er) => Err(er.into()),
        }
    }

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
        let resp = Requests::get(&self.options, &url, params).await;
        match resp {
            Ok(response) => {
                debug!("Transactions response: {:#?}", response);
                let list: TransactionList = response.json().await?;
                Ok(list)
            }
            Err(er) => Err(er.into()),
        }
    }
}
