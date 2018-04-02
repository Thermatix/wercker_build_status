use config_rs;

use std::collections::HashMap;
type Config = HashMap<String, String>;

pub fn load(config_file: String) -> Config {
    let mut settings = config_rs::Config::default();
    if config_file != "" {
        settings.merge(config_rs::File::with_name(config_file.as_str())).unwrap();
    }
    settings.merge(config_rs::Environment::with_prefix(::CONFIG_PREFIX)).unwrap();
    settings.try_into::<Config>().unwrap()
}
