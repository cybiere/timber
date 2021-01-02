use regex::Regex;
use std::path::PathBuf;
use serde::Deserialize;
use std::fs;

pub mod syslog_ng_to_regex {
    use regex::Regex;
    use serde::{Deserialize,Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Regex, D::Error>
        where
        D: Deserializer<'de>,
        {
            let src = String::deserialize(deserializer)?;
            let mut regex = String::from("^");
            let mut iter = src.chars();
            loop {
                let c = match iter.next(){
                    Some(c) => c,
                    None => break
                };
                if c == '$'{
                    let n = match iter.next(){
                        Some(c) => c,
                        None => break,
                    };
                    if n == '{' {
                        let mut key = String::new();
                        loop {
                            let k = match iter.next(){
                                Some(c) => c,
                                None => break,
                            }; 
                            if k == '}'{
                                break
                            }else{
                                key.push(k);
                            }
                        }
                        regex.push_str(match key.as_str(){
                            "YEAR" => r"(?P<year>\d{4})",
                            "MONTH" => r"(?P<month>\d{2})",
                            "DAY" => r"(?P<day>\d{2})",
                            "HOUR" => r"(?P<hour>\d{2})",
                            "MIN" => r"(?P<minute>\d{2})",
                            "SEC" => r"(?P<second>\d{2})",
                            "HOST" => r"(?P<host>\S+)",
                            "LEVEL" => r"(?P<level>[[:alpha:]]+)",
                            "MSGHDR" => r"(?P<process>[a-zA-Z0-9_\.-]+)(\[(?P<pid>\d+)\])?:\s",
                            "MSG" => r"(?P<message>.+)",
                            _ => "",
                        });
                    }else{
                        regex.push(c);
                        regex.push(n);
                    }
                }else if c != '\n'{
                    regex.push(c);
                }
            }
            regex.push('$');
            Ok(Regex::new(&regex).unwrap())
        }
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    #[serde(with = "syslog_ng_to_regex")]
    pub log_format : Regex,
}

impl Settings {
    pub fn load(config_file: PathBuf) -> Settings {
        let config = fs::read_to_string(config_file).expect("Unable to read config file");
        let settings: Settings = toml::from_str(&config).unwrap();
        settings
    }
}
