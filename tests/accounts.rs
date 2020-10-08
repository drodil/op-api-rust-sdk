#[cfg(test)]
mod accounts_tests {
    use op_api_sdk::apis::accounts::Accounts;
    use op_api_sdk::options::Options;
    use std::env;

    fn test_options() -> Options {
        Options::new_dev(env::var("X_API_KEY").unwrap())
    }

    #[tokio::test]
    async fn test_get_accounts() {
        let mut options = test_options();
        options.set_version("v3".to_string());
        let client = Accounts::new(options);

        // First test getting all accounts
        let resp = client.get_all().await;
        assert_eq!(true, resp.is_ok());

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
        let single_resp = client
            .get_account(original_account.account_id.clone())
            .await;
        assert_eq!(true, single_resp.is_ok());

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
    }
}
