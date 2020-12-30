use regex::Regex;
use crate::parser::LogLine;
use std::path::PathBuf;
use std::rc::Rc;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    name: String,
    process: String,
    regex: Regex,
//    triggers: Vec<Rc<Trigger>>
}

impl Rule {
    pub fn from_string(line : &str) -> Result<Rule, &str>{
        let fields: Vec<&str> = line.splitn(4,':').collect();
        if fields.len() != 4{
            return Err("Unable to parse rule");
        }
        Ok(Rule::new(fields[0],fields[1],fields[3]))
    }

    pub fn new(name: &str, process: &str, regex: &str) -> Rule{
        Rule{
            name: String::from(name),
            process: String::from(process),
            regex: Regex::new(regex).unwrap(),
            //triggers: (),
        }
    }

    pub fn load(rule_file_path: PathBuf) -> HashMap<String,Vec<Rule>>{
        let rule_file = File::open(rule_file_path).expect("Unable to read rules file");
        let mut rules = HashMap::<String,Vec<Rule>>::new();

        for line in BufReader::new(rule_file).lines(){
            let line_str = line.unwrap();
            if line_str.is_empty() || line_str.starts_with('#') {
                continue
            }
            let new_rule = match Rule::from_string(line_str.as_str()){
                Ok(rule) => rule,
                Err(msg) => {println!("{}: {}",msg, line_str); continue}
            };
            let per_process = rules.entry(new_rule.process.clone()).or_insert(Vec::new());
            per_process.push(new_rule);
        }
        rules
    }

    pub fn is_match(&self, line: &Rc<LogLine>) -> Option<MatchedLine> {
        if self.process != line.process(){
            return None
        }
        if self.regex.is_match(line.message()){
            let matched_line = MatchedLine{
                rule :self,
                line : Rc::clone(line),
                fields : None
            };
            self.alert(&matched_line);
            return Some(matched_line)
        }
        None
    }

    fn alert(&self, matched_line : &MatchedLine) -> (){
        println!("> Rule {} matched : \n\t{:?}",self.name, matched_line.line.raw())
    }
}

#[derive(Debug)]
pub struct MatchedLine<'a> {
    rule: &'a Rule,
    line: Rc<LogLine>,
    fields: Option<HashMap<String,String>>
}
