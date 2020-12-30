use regex::Regex;
use crate::parser::LogLine;

#[derive(Debug)]
pub struct Rule {
    name: String,
    process: String,
    regex: Regex,
}

impl Rule {
    pub fn new(name: &str, process: &str, regex: &str) -> Rule{
        Rule{
            name: String::from(name),
            process: String::from(process),
            regex: Regex::new(regex).unwrap(),
        }
    }

    pub fn name(&self) -> &str{
        &self.name
    }

    pub fn is_match(&self, line: &LogLine) -> bool {
        if self.process != line.process(){
            return false
        }
        self.regex.is_match(line.message())
    }
}