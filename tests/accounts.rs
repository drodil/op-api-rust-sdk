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
        let resp = client.get_all().await;
        assert_eq!(true, resp.is_ok());

        let accounts = resp.unwrap();
        assert_eq!(1, accounts.accounts.len());

        match accounts.accounts.get(0) {
            Some(account) => {
                assert_eq!(false, account.account_id.is_empty());
                assert_eq!(false, account.name.is_empty());
                assert_eq!(false, account.currency.is_empty());
                assert_eq!(false, account.identifier_scheme.is_empty());
                assert_eq!(false, account.identifier.is_empty());
                assert_eq!(false, account.servicer_scheme.is_empty());
                assert_eq!(false, account.servicer_identifier.is_empty());
            }
            None => panic!("No accounts received from endpoint!"),
        };
    }
}
