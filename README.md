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

## License

Copyright (C) 2016, Richard Pelletier

```
MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
