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
        let fields: Option<HashMap<String,String>>;
        let found: bool;
        if self.regex.captures_len() > 1 {
            found = match self.regex.captures(line.message()){
                None => { fields = None; false},
                Some(caps) => {
                    let mut fields_content = HashMap::new();
                    for capture_name in self.regex.capture_names(){
                        if let Some(name) = capture_name{
                            fields_content.insert(String::from(name),String::from(caps.name(name).unwrap().as_str()));
                        }
                    }
                    fields = Some(fields_content);
                    true
                }
            };
        }else{
            found = self.regex.is_match(line.message());
            fields = None;
        }
        if found{
            let matched_line = MatchedLine{
                rule :self,
                line : Rc::clone(line),
                fields : fields
            };
            self.alert(&matched_line);
            return Some(matched_line)
        }
        None
    }

    fn alert(&self, matched_line : &MatchedLine) -> (){
        println!("> Rule {} matched : \n{:#?}",self.name, matched_line)
    }
}

#[derive(Debug)]
pub struct MatchedLine<'a> {
    rule: &'a Rule,
    line: Rc<LogLine>,
    fields: Option<HashMap<String,String>>
}
