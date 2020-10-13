use op_api_sdk::client::Client;
/// This example show how to use Accounts client for
/// fetching accounts and transaction details for each
/// account.
///
/// The example requires API key from https://op-developer.fi for
/// the sandbox environment. API key should be passed to the
/// example as a command line argument.
///
/// How to run this example:
/// ```bash
/// cargo run --example accounts <API_KEY>
/// ```
use op_api_sdk::model::accounts::{
    AccountList, TransactionList, TransactionParams, TransactionParty,
};
use op_api_sdk::options::Options;
use std::env;

const NONE_STRING: &str = "None";

/// Prints transaction party information.
fn print_transaction_party(party: Option<TransactionParty>) {
    match party {
        Some(p) => {
            println!("    Name: {}", p.account_name);
            println!("    IBAN: {}", p.account_identifier);
            println!("    Servicer: {}", p.servicer_identifier);
        }
        None => {
            println!("    No information");
        }
    }
}

/// Prints transactions from transaction list.
fn print_transactions_with_details(transactions: TransactionList) {
    if transactions.transactions.is_empty() {
        println!("  No transactions found");
        return;
    }

    for tr in transactions.transactions {
        println!("****************");
        println!("  ID: {}", tr.transaction_id);
        println!("  Account ID: {}", tr.account_id);
        println!(
            "  Archive ID: {}",
            tr.archive_id.unwrap_or_else(|| NONE_STRING.to_string())
        );
        println!(
            "  Reference: {}",
            tr.reference.unwrap_or_else(|| NONE_STRING.to_string())
        );
        println!(
            "  Message: {}",
            tr.message.unwrap_or_else(|| NONE_STRING.to_string())
        );
        println!("  Amount: {}{}", tr.amount, tr.currency);
        println!("  Type: {}", tr.credit_debit_indicator);
        println!("  Account balance: {}", tr.account_balance);
        println!("  Creditor:");
        print_transaction_party(tr.creditor);
        println!("  Debtor:");
        print_transaction_party(tr.debtor);
        println!("  Booking: {}", tr.booking_datetime);
        println!("  Balanced: {}", tr.value_datetaime);
        println!(
            "  Status: {}",
            tr.status.unwrap_or_else(|| "UNKNOWN".to_string())
        );
    }
}

/// Prints accounts in the list and fetches 5 last transactions for
/// each account using the Accounts client. This function must be async
/// as we are calling another async function for fetching the transactions
/// for the accounts.
async fn print_accounts_with_details(client: &Client, accounts: AccountList) {
    println!("Found {} accounts:", accounts.accounts.len());
    for acc in accounts.accounts {
        println!("-----------------");
        println!("ID: {}", acc.account_id);
        println!("Name: {}", acc.name);
        println!(
            "Nickname: {}",
            acc.nickname.unwrap_or_else(|| NONE_STRING.to_string())
        );
        println!(
            "Balance: {}{}",
            acc.balance.unwrap_or(std::f64::NAN),
            acc.currency
        );
        println!("IBAN: {}", acc.identifier);
        println!("Last 5 transactions:");

        let params = TransactionParams::default().with_page_size(5);

        // Fetch transactions for the account here.
        let trans_resp = client.transactions(acc.account_id, Some(params)).await;
        match trans_resp {
            Ok(transactions) => {
                print_transactions_with_details(transactions);
            }
            Err(e) => {
                println!("  Failed to fetch transactions for account: {}", e);
            }
        };
    }
}

/// Example of getting accounts for the user. Uses OP API sandbox.
/// API key must be given as command line argument.
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Please provide API key as parameter");
        return;
    }

    let options = Options::new_dev(args[1].clone());
    options.set_version("v3".to_string());
    let client = Client::new(options);
    let resp = client.accounts().await;
    match resp {
        Ok(accounts) => {
            print_accounts_with_details(&client, accounts).await;
        }
        Err(e) => {
            println!("Failed to get accounts: {}", e);
        }
    };
}
