use regex::Regex;
use crate::parser::LogLine;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rule {
    name: String,
    process: String,
    regex: Regex,
//    triggers: Vec<Rc<Trigger>>
}

impl Rule {
    pub fn new(name: &str, process: &str, regex: &str) -> Rule{
        Rule{
            name: String::from(name),
            process: String::from(process),
            regex: Regex::new(regex).unwrap(),
            //triggers: (),
        }
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
