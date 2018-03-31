extern crate config;
extern crate curl;
// extern crate serde_json;
// #[macro_use]
// extern crate serde_derive;

use std::env;
use std::collections::HashMap;

// use std::io::{stdout, Write};

use curl::easy::Easy;

const API_VERSION: &str = "v3";

type Config = HashMap<String, String>;

fn main() {

    let settings =
        match env::args().nth(1) {
        Some(file_path) => load_config(file_path),
        None => load_config("".to_string())
    };

    let keys: [&str;3] = ["api_key", "user_name", "pipeline"];

    match keys.iter().position( |key| {
        !settings.contains_key(&key.to_string())
    }) {
        Some(i) => println!( "No `{}` detected in config or env (WERCKER_{}) variables",
            keys[i],keys[i].to_uppercase()),
        None    => {
            println!("{:?}",settings);
            let mut client = set_up_client(&settings["api_key"]);
            get_runs(&mut client);

        }
    };
    
    
}

fn set_up_client(token: &String) -> Easy {

    use curl::easy::List;

    let mut easy = Easy::new();
    let mut list = List::new();

    list.append(&format!("Authorization: Bearer {}",token)).unwrap();
    easy.http_headers(list).unwrap();
    easy
}

fn load_config(config_file: String) -> Config {
    println!("{}",config_file);
    let mut settings = config::Config::default();
    if config_file != "" {
        settings.merge(config::File::with_name(config_file.as_str())).unwrap();
    }
    settings.merge(config::Environment::with_prefix("WERCKER")).unwrap();
    settings.try_into::<Config>().unwrap()
}


fn get_runs(curl: &mut Easy) { // -> String {
    use std::str;
    let mut data = Vec::new();

    curl.url(runs_url().as_str()).unwrap();
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{:?}",str::from_utf8(&data).unwrap());
    // data.to_string()
    // dst.to_string()
}

fn runs_url() -> String {
    build_url("runs")
}

fn build_url(endpoint: &str) -> String {
    format!("https://app.wercker.com/api/{}/{}",API_VERSION,endpoint)
}

// fn process(){
    // use serde_json::{Value, Error};
// }
