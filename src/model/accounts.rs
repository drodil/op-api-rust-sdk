//! Models required for
//! [AccountsV3](https://op-developer.fi/docs/api/3Oo5zCujXGw2SGEi00skug/OP%20Accounts%20V3.0%20API)
//! API

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Link inside the results of Accounts API.
#[derive(Deserialize, Debug)]
pub struct Link {
    pub href: String,
}

/// Describes a single Account in response.
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

/// Describes a list of Accounts in accounts response.
#[derive(Deserialize, Debug)]
pub struct AccountList {
    pub accounts: Vec<Account>,
}

/// Optional parameters to fetch transactions.
#[derive(Serialize, Debug, Default)]
pub struct TransactionParams {
    /// ISO 8601-compatible date-time string representing the earliest date-time from which
    /// transactions will be queried. Timezone must not be set. Set time to to 00:00:00 for
    /// Date-only queries.
    #[serde(rename = "fromBookingDateTime")]
    pub from_booking_datetime: Option<DateTime<Utc>>,
    /// ISO 8601-compatible date-time string representing the latest date-time up to which
    /// transactions will be queried. Timezone must not be set. Set time to 00:00:00 for Date-only
    /// queries.
    #[serde(rename = "toBookingDateTime")]
    pub to_booking_datetime: Option<DateTime<Utc>>,
    /// Number of transactions to be returned per each page.
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
    /// Paging token used to retrieve the next page of data. Tokens are available in the links
    /// located in the _links object.
    #[serde(rename = "forwardPagingToken")]
    pub forward_paging_token: Option<String>,
}

/// Implementation of the TransactionParams.
impl TransactionParams {
    /// Sets fromBookingDateTime parameter.
    pub fn with_from_booking_datetime(mut self, datetime: DateTime<Utc>) -> Self {
        self.from_booking_datetime = Some(datetime);
        self
    }

    /// Sets toBookingDateTime parameter.
    pub fn with_to_booking_datetime(mut self, datetime: DateTime<Utc>) -> Self {
        self.to_booking_datetime = Some(datetime);
        self
    }

    /// Sets pageSize parameter.
    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Sets forwardPagingToken parameter.
    pub fn with_forward_paging_token(mut self, token: String) -> Self {
        self.forward_paging_token = Some(token);
        self
    }
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

/// Describes a single Transaction for Account in transactions response.
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

/// Describes links in the Transactions object in transactions response.
#[derive(Deserialize, Debug)]
pub struct TransactionListLinks {
    pub next: Option<Link>,
}

/// Describes a list of Transactions in transactions response.
#[derive(Deserialize, Debug)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
    #[serde(rename = "_links")]
    pub links: TransactionListLinks,
}
