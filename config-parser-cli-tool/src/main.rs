mod parser;
mod config;
mod utils;

use clap::{App, Arg};
use parser::load_config;
use config::{query_key, set_key, save_config};


fn main() {
    let matches = App::new("Config Parser CLI")
        .version("1.0")
        .author("Your Name")
        .about("Parse, query, and modify config files in JSON, TOML, or YAML")
        .arg(Arg::new("file").about("Path to the config file").required(true))
        .arg(Arg::new("format").about("Config file format").required(false))
        .arg(Arg::new("query").about("Dot-notation key to query").required(false))
        .arg(Arg::new("set").about("Key-value pair to set").required(false))
        .arg(Arg::new("convert-to").about("Format to convert to").required(false))
        .get_matches();

    let file_path = matches.value_of("file").unwrap();
    let format = matches.value_of("format");
    let query = matches.value_of("query");
    let set = matches.value_of("set");
    let convert_to = matches.value_of("convert-to");

    let mut config = load_config(file_path, format).expect("Failed to load config");

    if let Some(query_key) = query {
        if let Some(value) = query_key(&config, query_key) {
            println!("Value: {}", value);
        } else {
            eprintln!("Key not found");
        }
    }

    if let Some(set_value) = set {
        let (key, value) = set_value.split_once('=').expect("Invalid set format");
        set_key(&mut config, key, value).expect("Failed to set key");
        save_config(file_path, &config, format).expect("Failed to save config");
    }

    if let Some(target_format) = convert_to {
        save_config(file_path, &config, Some(target_format)).expect("Failed to convert config");
        println!("Config converted to {}", target_format);
    }
}