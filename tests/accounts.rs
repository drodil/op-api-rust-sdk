#[cfg(test)]
#[cfg(feature = "accounts")]
mod accounts_tests {
    use op_api_sdk::client::Client;
    use op_api_sdk::model::accounts::*;
    use op_api_sdk::options::Options;
    use std::env;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    async fn test_accounts() {
        init();
        let options = Options::new_dev(env::var("X_API_KEY").unwrap());
        options.set_version("v3".to_string());
        let client = Client::new(options);

        // First test getting all accounts
        let resp = client.accounts().await;
        assert_eq!(true, resp.is_ok(), "{:?}", resp.err());

        let accounts = resp.unwrap();
        assert_eq!(1, accounts.accounts.len());

        let account = match accounts.accounts.get(0) {
            Some(account) => {
                assert_eq!(false, account.account_id.is_empty());
                assert_eq!(false, account.name.is_empty());
                assert_eq!(false, account.currency.is_empty());
                assert_eq!(false, account.identifier_scheme.is_empty());
                assert_eq!(false, account.identifier.is_empty());
                assert_eq!(false, account.servicer_scheme.is_empty());
                assert_eq!(false, account.servicer_identifier.is_empty());
                Some(account)
            }
            None => panic!("No accounts received from endpoint!"),
        };

        // Now try to fetch single account from the accounts list
        let original_account = account.unwrap();
        let single_resp = client.account(original_account.account_id.clone()).await;
        assert_eq!(true, single_resp.is_ok(), "{:?}", single_resp.err());

        let single_account = single_resp.unwrap();
        assert_eq!(original_account.name, single_account.name);
        assert_eq!(original_account.currency, single_account.currency);
        assert_eq!(
            original_account.identifier_scheme,
            single_account.identifier_scheme
        );
        assert_eq!(original_account.identifier, single_account.identifier);
        assert_eq!(
            original_account.servicer_scheme,
            single_account.servicer_scheme
        );
        assert_eq!(
            original_account.servicer_identifier,
            single_account.servicer_identifier
        );

        // Then try to fetch transactions
        let params = TransactionParams {
            forward_paging_token: None,
            from_booking_datetime: None,
            page_size: Some(5),
            to_booking_datetime: None,
        };
        let trans_resp = client
            .transactions(original_account.account_id.clone(), Some(params))
            .await;
        assert_eq!(true, trans_resp.is_ok(), "{:?}", trans_resp.err());

        let transactions = trans_resp.unwrap();
        assert_ne!(0, transactions.transactions.len());

        for trans in transactions.transactions.iter() {
            assert_eq!(false, trans.transaction_id.is_empty());
            assert_eq!(false, trans.amount.is_empty());
        }
    }
}
