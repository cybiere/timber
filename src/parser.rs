use std::error::Error;
use std::str::FromStr;
use std::io;

pub fn read_line() -> Option<String> {
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

#[derive(Debug)]
pub struct LogLine {
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

use crate::settings::Settings;

impl LogLine {
    pub fn process(&self) -> &str{
        &self.process
    }

    pub fn raw(&self) -> &str{
        &self.raw
    }

    pub fn message(&self) -> &str{
        &self.message
    }

    fn clean_value<T: FromStr>(caps: &regex::Captures,key :&str) -> Result<Option<T>,String> {
        match caps.name(key) {
            Some(value) => match value.as_str().parse::<T>(){
                Ok(value) => Ok(Some(value)),
                Err(_) => Err(format!("Could not parse '{:?}'",value)),
            },
            None => Ok(None),
        }
    }

    pub fn from_string(line: &str, settings :&Settings) -> Result<LogLine,Box<dyn Error>> {
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