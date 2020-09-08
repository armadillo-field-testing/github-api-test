# Github API Tests with Rust

This modest repo is a place to store test code while I learn Rust.

## Prerequisites

* A functional Rust development environment. See: https://www.rust-lang.org/tools/install .
* A Github Personal Access Token, stored in an environment variable named 'GITHUB_TOKEN'. See: https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token for more information on these tokens.

For Github REST API docs, see: https://docs.github.com/en/rest

# How to Use

After cloning this repo, compile it using cargo:
```
$ cargo build --release
```

To run, use:
```
$ ./target/release/github-api-test
```

You should receive a JSON object in return detailing your current API call limits. An error will be returned if something went wrong. Piping the output through a utility like 'jq' can be helpful. See: https://github.com/stedolan/jq .

# Built With

* [Rust](https://www.rust-lang.org) Designed by	Graydon Hoare

## Author

**Rick Pelletier** - [Gannett Co., Inc. (USA Today Network)](https://www.usatoday.com/)
