extern crate config as config_rs;
extern crate curl;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod config;
mod client;
mod json;

use std::env;
use json::Runs;

const API_VERSION: &str = "v3";
const CONFIG_PREFIX: &str = "WERCKER";


fn main() {
    // first get config
    let settings =
        match env::args().nth(1) {
        Some(file_path) => config::load(file_path),
        None => config::load("".to_string())
    };

    // check if config options exists
    let keys: [&str;3] = ["token", "author", "pipeline_id"];

    match keys.iter().position( |key| {
        !settings.contains_key(&key.to_string())
    }) {
        Some(i) => println!( "No `{}` detected in config or env ({}_{}) variables",
            keys[i], CONFIG_PREFIX, keys[i].to_uppercase()),
        None    => {
            // set up curl client
            let mut client = client::set_up(&settings["token"]);
            // get last 20 runs (max you an get) and deserialize json into structs
            let runs: Runs = serde_json::from_str(client::get_runs(&mut client,&settings["pipeline_id"]).as_str()).unwrap();
            // find first matching username as most recent is first
            match runs.iter().find( |run| {
                &run.user.meta.username.to_lowercase() == &settings["author"].to_lowercase()
            }) {
                // print out status and result
                Some(run) => println!("{}:{}",&run.status,&run.result),
                None => print!("{}","None Found")

            }
        }
    };
}








