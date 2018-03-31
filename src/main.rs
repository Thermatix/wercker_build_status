extern crate config;
use std::env;
use std::collections::HashMap;

type Config = HashMap<String, String>;

fn main() {
    let settings =
        match env::args().nth(1) {
        Some(file_path) => load_config(file_path),
        None => load_config("".to_string())
    };

    if !settings.contains_key("api_key") {
        println!("No `api_key` detected in config")
    } else if !settings.contains_key("user_name") {
        println!("No `user_name` detected in config")
    }
    else if !settings.contains_key("pipeline") {
        println!("No `pipline` detected in config")
    } else {
        println!("{:?}",settings);
    }
}

fn load_config(config_file: String) -> Config {
    let mut settings = config::Config::default();
    if config_file != "" {
        settings.merge(config::File::with_name(config_file.as_str())).unwrap();
    }
    settings.merge(config::Environment::with_prefix("WERCKER")).unwrap();
    settings.try_into::<HashMap<String, String>>().unwrap()
}
