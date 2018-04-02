use config_rs;

use std::collections::HashMap;
type Config = HashMap<String, String>;

pub fn load(config_file: String) -> Config {
    // create config object
    let mut settings = config_rs::Config::default();
    // check if config_file was supplied, if it was then load it    
    if config_file != "" {
        settings.merge(config_rs::File::with_name(config_file.as_str())).unwrap();
    }
    //check for and add env variables
    settings.merge(config_rs::Environment::with_prefix(::CONFIG_PREFIX)).unwrap();
    settings.try_into::<Config>().unwrap()
}
