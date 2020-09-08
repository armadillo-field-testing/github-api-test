extern crate reqwest;

use std::env;
use std::process;
use github_rs::client::{Executor, Github};
use serde_json::Value;

/*
ref: https://developer.github.com/v3/
ref: https://github.com/github-rs/github-rs
ref: https://github.com/github-rs/github-rs/blob/master/docs/endpoints.md
*/

fn main() {
    let access_token = env::var("GITHUB_TOKEN"); // personal access token stored as env var

    if access_token.is_ok() {
        let client = Github::new(access_token.unwrap()).unwrap();
        let rate_limit_data = client.get().rate_limit().execute::<Value>();

        match rate_limit_data {
            Ok((_headers, _status, json)) => {
                // println!("{:#?}", headers);
                // println!("{}", status);

                if let Some(json) = json {
                    println!("{}", json);
                }
            },
            Err(e) => println!("{}", e)
        }

    } else {
        println!("ERROR: Env var GITHUB_TOKEN not found");
        process::exit(1);
    }

}
