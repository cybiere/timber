use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct LogParseError;

impl fmt::Display for LogParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cannot parse log line")
    }
}

#[derive(Debug)]
struct LogLine {
    year: Option<u32>,
    month: Option<u8>,
    day: Option<u8>,
    hour: Option<u8>,
    minute: Option<u8>,
    second: Option<u8>,
    level: Option<String>,
    process: Option<String>,
    pid: Option<u32>,
    message: Option<String>,
}

impl LogLine {
    fn clean_value<T: FromStr>(caps: &regex::Captures,key :&str) -> Option<T> {
        match caps.name(key) {
            Some(value) => match value.as_str().parse::<T>(){
                Ok(value) => Some(value),
                Err(_) => None,
            },
            None => None,
        }
    }

    fn from_string(line: &str, format :&Regex) -> Option<LogLine> {
        let caps = match format.captures(&line){
            Some(caps) => caps,
            None => return None,
        };
        let logline = LogLine{
            year: LogLine::clean_value(&caps, "year"),
            month: LogLine::clean_value(&caps, "month"),
            day: LogLine::clean_value(&caps, "day"),
            hour: LogLine::clean_value(&caps, "hour"),
            minute: LogLine::clean_value(&caps, "minute"),
            second: LogLine::clean_value(&caps, "second"),
            level: LogLine::clean_value(&caps, "level"),
            process: LogLine::clean_value(&caps, "process"),
            pid: LogLine::clean_value(&caps, "pid"),
            message: LogLine::clean_value(&caps, "message"),
        };
        Some(logline)
    }
}

fn main() {
    let format = Regex::new(r"(?x)^
    (?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})\s
    (?P<hour>\d{2}):(?P<minute>\d{2}):(?P<second>\d{2})\s
    (?P<level>[[:alpha:]]+)\s
    (?P<process>[[:alpha:]]+)(\[(?P<pid>\d+)\])?:(\s
    (?P<message>.+))?$").unwrap();
    let line = "2020-11-23 01:37:13 info sshd[57587]: Disconnected from user borg 192.168.44.2 port 33598";
    let line2 = "This is garbage";
    let line3 = "2020-11-28 16:43:06 info kernel: device vethee57d00 left promiscuous mode";
    let line4 = "2020-11-12 15:59:06 debug root:";
    let line5 = "aze-11-12 15:59:06 debug root:";

    match LogLine::from_string(&line,&format){
        Some(logline) => println!("Log line : {:?}",logline),
        None => println!("Failed to parse line"),
    };
    match LogLine::from_string(&line2,&format){
        Some(logline) => println!("Log line : {:?}",logline),
        None => println!("Failed to parse line"),
    };
    match LogLine::from_string(&line3,&format){
        Some(logline) => println!("Log line : {:?}",logline),
        None => println!("Failed to parse line"),
    };
    match LogLine::from_string(&line4,&format){
        Some(logline) => println!("Log line : {:?}",logline),
        None => println!("Failed to parse line"),
    };
    match LogLine::from_string(&line5,&format){
        Some(logline) => println!("Log line : {:?}",logline),
        None => println!("Failed to parse line"),
    };

}
