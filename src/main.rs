use regex::Regex;
use std::str::FromStr;
use std::io;
use std::fs;
use std::error::Error;
use std::collections::HashMap;
use structopt::StructOpt;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str), short, long)]
    config: Option<PathBuf>,
}

#[derive(Debug)]
struct MatchRule {
    name: String,
    process: String,
    regex: Regex,
}

impl MatchRule {
    fn is_match(&self, line: &LogLine) -> bool {
        if self.process != line.process{
            return false
        }
        self.regex.is_match(line.message.as_str())
    }
}

#[derive(Debug)]
struct LogLine {
    year: u32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    level: String,
    process: String,
    host: String,
    pid: Option<u32>,
    message: String,
    raw: String,
}

impl LogLine {
    fn clean_value<T: FromStr>(caps: &regex::Captures,key :&str) -> Result<Option<T>,Box<dyn Error>> {
        match caps.name(key) {
            Some(value) => match value.as_str().parse::<T>(){
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(format!("Could not parse '{:?}'",value).into()),
            },
            None => Ok(None),
        }
    }

    fn from_string(line: &str, settings :&Settings) -> Result<LogLine,Box<dyn Error>> {
        let caps = match settings.log_format.captures(&line){
            Some(caps) => caps,
            None => return Err("Line does not match specified format.".into()),
        };
        let year = LogLine::clean_value(&caps, "year")?.unwrap();
        let month = LogLine::clean_value(&caps, "month")?.unwrap();
        let day = LogLine::clean_value(&caps, "day")?.unwrap();
        let hour = LogLine::clean_value(&caps, "hour")?.unwrap();
        let minute = LogLine::clean_value(&caps, "minute")?.unwrap();
        let second = LogLine::clean_value(&caps, "second")?.unwrap();
        let level = LogLine::clean_value(&caps, "level")?.unwrap();
        let process = LogLine::clean_value(&caps, "process")?.unwrap();
        let host = LogLine::clean_value(&caps, "host")?.unwrap();
        let pid = LogLine::clean_value(&caps, "pid")?;
        let message = LogLine::clean_value(&caps, "message")?.unwrap();

        Ok(LogLine::build(year, month, day, hour, minute, second, host, level, process, pid, message, line))
    }

    fn build(year :u32, month :u8, day :u8, hour :u8, minute :u8, second :u8, host :String, level :String, process :String, pid :Option<u32>, message :String, line : &str) -> LogLine{
        LogLine {
            year,
            month,
            day,
            hour,
            minute,
            second,
            level,
            host,
            process,
            pid,
            message,
            raw: String::from(line),
        }
    }
}

fn read_line() -> Option<String> {
    let mut line = String::new();
    let readbytes = match io::stdin().read_line(&mut line){
        Ok(n) => n,
        Err(error) => { println!("Error : {}",error); return None }
    };
    if readbytes == 0 {
        return None
    }
    if line.ends_with('\n') {
        line.pop();
        if line.ends_with('\r') {
            line.pop();
        }
    }
    return Some(line)
}

pub mod format{
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
                                "HOST" => r"(?P<host>[[:alpha:]]+)",
                                "LEVEL" => r"(?P<level>[[:alpha:]]+)",
                                "MSGHDR" => r"(?P<process>[[:alpha:]]+)(\[(?P<pid>\d+)\])?:\s",
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
}

#[derive(Debug, Deserialize)]
struct Settings {
    #[serde(with = "format::syslog_ng_to_regex")]
    log_format : Regex,
}

impl Settings {
    fn load(config_file: PathBuf) -> Settings {
        let config = fs::read_to_string(config_file).expect("Unable to read config file");
        let settings: Settings = toml::from_str(&config).unwrap();
        settings
    }
}

fn main() {
    let opt = Opt::from_args();
    let settings = Settings::load(opt.config.unwrap_or(PathBuf::from("/etc/timber/timber.conf")));

    let mut rules = HashMap::new();
    rules.insert(String::from("sshd"),Vec::new());
    rules.get_mut("sshd").unwrap().push(MatchRule{
        name: String::from("ssh-accepted"),
        process: String::from("sshd"),
        regex: Regex::new(r"Accepted").unwrap(),
    });
    /*   
         rules.get_mut("sshd").unwrap().push(MatchRule{
         name: String::from("ssh-disconnect"),
         process: String::from("sshd"),
         regex: Regex::new(r"Disconnect").unwrap(),
         });
         */

    loop {
        let line = match read_line(){
            Some(line) => line,
            None => break
        }; 
        let logline = match LogLine::from_string(&line,&settings){
            Ok(logline) => logline,
            Err(_) => continue,
        };
        let process_rules = match rules.get(&logline.process){
            Some(v) => v,
            None => continue,
        };
        for rule in process_rules {
            if rule.is_match(&logline){
                println!(">> Match on rule {} for host {}. Raw line is: \n\t{}", rule.name, logline.host, logline.raw);
            }
        }

    }
}
