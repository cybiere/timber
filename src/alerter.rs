use std::path::PathBuf;
use serde::Deserialize;
use std::fs;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Trigger{
    name: String,
    rules: Vec<HashMap<String,String>>,
    integrity: Vec<Vec<String>>,
    keys: Vec<String>,
    init: HashMap<String,String>
}

impl Trigger{
    pub fn load(trigger_file_path: PathBuf) -> (){
        println!("Loading triggers from {:?}", trigger_file_path);
        let triggers_str = fs::read_to_string(trigger_file_path).expect("Unable to read config file");
        let triggers: HashMap<String,Vec<Trigger>> = toml::from_str(&triggers_str).unwrap();
        println!("{:#?}",triggers);
    }
}