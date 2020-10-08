# op-api-rust-sdk

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/drodil/op-api-rust-sdk/blob/master/LICENSE)
[![Contributors](https://img.shields.io/github/contributors/drodil/op-api-rust-sdk.svg?style=flat)]()
[![Issues](https://img.shields.io/github/issues-raw/drodil/op-api-rust-sdk.svg?maxAge=25000)](https://github.com/drodil/op-api-rust-sdk/issues)
[![PRs](https://img.shields.io/github/issues-pr/drodil/op-api-rust-sdk.svg?style=flat)](https://github.com/drodil/op-api-rust-sdk/pulls)

Rust SDK for [OP REST API](https://op-developer.fi/)

## Installation

To be published to [crates.io](https://crates.io/)

## Usage

See apis crate for all available clients. Example of getting account data:

```rust
use op_api_sdk::apis::accounts::Accounts;
use op_api_sdk::options::Options;
use tokio;

#[tokio::main]
async fn main() {
    let mut options = Options::new_dev(String::from("X_API_KEY"));
    options.set_version("v3".to_string());
    let accounts = Accounts::new(options).accounts().await.unwrap();
    println!("{:?}", accounts);
}
```

See [requests](https://op-developer.fi/docs/#user-content-requests) for required headers.

For further reading, please see our API [documentation](https://op-developer.fi/docs/)

## Developing

Few guidelines for developing this library:

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Keep formatting consistent by using `cargo fmt` for all changes
- Use `cargo clippy` to find possible lint errors and warnings
- Always add tests to your functionality, prefer TDD. Use /tests/ folder.
- Always run tests before pushing to remote

### Running tests

Tests depend to real sandbox data.

- Register at https://op-developer.fi/developers/register
- Create an app that has access to all sandbox products (Mobility, Banking, etc)
- Run tests with `X_API_KEY=<your api key> cargo test`

To enable debug logging from the library set up the RUST_LOG environment
variable

    export RUST_LOG=op_api_sdk=debug
