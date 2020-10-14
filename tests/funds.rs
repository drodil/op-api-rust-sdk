#[cfg(test)]
mod funds_tests {
    use op_api_sdk::client::Client;
    use op_api_sdk::options::Options;
    use std::env;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test]
    #[ignore]
    async fn test_funds() {
        init();
        let options = Options::new_dev(env::var("X_API_KEY").unwrap());
        options.set_version("v1".to_string());
        let client = Client::new(options);

        // First test getting funds
        let resp = client.funds().await;
        assert_eq!(true, resp.is_ok(), "{:?}", resp.err());

        let funds = resp.unwrap();
        assert_eq!(false, funds.fund_name.is_empty());
    }
}
