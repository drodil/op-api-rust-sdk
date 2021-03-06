# op-api-rust-sdk

[![MIT License](https://img.shields.io/apm/l/atomic-design-ui.svg?)](https://github.com/drodil/op-api-rust-sdk/blob/master/LICENSE)
[![Contributors](https://img.shields.io/github/contributors/drodil/op-api-rust-sdk.svg?style=flat)]()
[![Issues](https://img.shields.io/github/issues-raw/drodil/op-api-rust-sdk.svg?maxAge=25000)](https://github.com/drodil/op-api-rust-sdk/issues)
[![PRs](https://img.shields.io/github/issues-pr/drodil/op-api-rust-sdk.svg?style=flat)](https://github.com/drodil/op-api-rust-sdk/pulls)

Rust SDK for [OP REST API](https://op-developer.fi/)

## Installation

To use this library, simply add the following to your Cargo.toml:

```rust
op-api-sdk = "0.1.0"
```

See https://crates.io/crates/op-api-sdk for more versions and details of the crate.

## Usage

See apis crate for all available clients. Example of getting account data:

```rust
use op_api_sdk::client::Client;
use op_api_sdk::options::Options;

#[tokio::main]
async fn main() {
    let options = Options::new_dev(String::from("X_API_KEY"));
    options.set_version("v3".to_string());
    let accounts = Client::new(options).accounts().await.unwrap();
    println!("{:?}", accounts);
}
```

Additional examples available in the
[Examples](https://github.com/drodil/op-api-rust-sdk/tree/main/examples)
directory.

See [requests](https://op-developer.fi/docs/#user-content-requests) for required headers.

For further reading, please see our API [documentation](https://op-developer.fi/docs/)

## Developing

Few guidelines for developing this library:

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Keep formatting consistent by using `cargo fmt` for all changes
- Use `cargo clippy --workspace --all-targets --verbose --all-features`
  to find possible lint errors and warnings
- Always add tests to your functionality, prefer TDD. Use /tests/ folder.
- Always run tests with `cargo test` before pushing to remote
- Check that you have documented all public functionality with `cargo doc
  --open`

### Running tests

Tests depend to real sandbox data.

- Register at https://op-developer.fi/developers/register
- Create an app that has access to all sandbox products (Mobility, Banking, etc)
- Run tests with `X_API_KEY=<your api key> cargo test`

To enable debug logging from the library set up the RUST_LOG environment
variable

    export RUST_LOG=op_api_sdk=debug
