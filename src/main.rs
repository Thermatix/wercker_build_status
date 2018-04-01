extern crate config;
extern crate curl;
extern crate serde_json;

// #[macro_use]
// extern crate serde_derive;

use std::env;
use std::collections::HashMap;

// use std::io::{stdout, Write};

use curl::easy::Easy;

use serde_json::{Value, Error};

const API_VERSION: &str = "v3";
const CONFIG_PREFIX: &str = "WERCKER";
// const LAST_BUILD: usize = 18;
const FIRST: usize = 0;

type Config = HashMap<String, String>;

fn main() {

    let settings =
        match env::args().nth(1) {
        Some(file_path) => load_config(file_path),
        None => load_config("".to_string())
    };

    let keys: [&str;3] = ["token", "author", "pipeline_id"];

    match keys.iter().position( |key| {
        !settings.contains_key(&key.to_string())
    }) {
        Some(i) => println!( "No `{}` detected in config or env ({}_{}) variables",
            keys[i], CONFIG_PREFIX, keys[i].to_uppercase()),
        None    => {
            let mut client = set_up_client(&settings["token"]);
            let runs: Value = serde_json::from_str(get_runs(&mut client,&settings["author"],
                                &settings["pipeline_id"]).as_str()).unwrap();
            let runs_len = runs.as_array().iter().len();
            println!("{}",runs[FIRST]);
            // println!("{}",runs);
            // println!("{}", runs[( IDX + runs_len - 1) % runs_len]);
            // match runs[FIRST]["status"] {
            //     "finished" => println!("{}",runs[FIRST]["result"]),
            //     _ => println!("*{}",runs[FIRST]["result"])
            // }
            // println!("{:?}:{}", runs[FIRST]["status"],runs[FIRST]["result"]);
            println!("{}",runs[FIRST]["result"]);

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
    let mut settings = config::Config::default();
    if config_file != "" {
        settings.merge(config::File::with_name(config_file.as_str())).unwrap();
    }
    settings.merge(config::Environment::with_prefix(CONFIG_PREFIX)).unwrap();
    settings.try_into::<Config>().unwrap()
}



fn get_runs(curl: &mut Easy, author: &String, pipline_id: &String)  -> String {
    let url = url_runs(&author, &pipline_id);
    println!("{}",&url);
    get(curl,url )
}

fn get(curl: &mut Easy, url: String) -> String {
    use std::str;

    let mut data = Vec::new();

    curl.url(url.as_str()).unwrap();
    {
        let mut transfer = curl.transfer();
        transfer.write_function(|new_data| {
            data.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    str::from_utf8(&data).unwrap().to_string()
}


fn url_runs(author: &String, pipline_id: &String) -> String {
    // format!("{}?author={}&pipelineId={}",build_url("runs"),&author,&pipline_id)
    format!("{}?author={}&pipelineId={}",build_url("runs"),&author,&pipline_id)
    // format!("{}?pipelineId={}",build_url("runs"),&pipline_id)
}

fn build_url(endpoint: &str) -> String {
    format!("https://app.wercker.com/api/{}/{}",API_VERSION, endpoint)
}

// fn process(){
    // use serde_json::{Value, Error};
// }
