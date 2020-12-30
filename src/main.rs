use std::collections::HashMap;
use structopt::StructOpt;
use std::path::PathBuf;

mod settings;
mod parser;
mod matcher;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str), short, long)]
    config: Option<PathBuf>,
}

/*
#[derive(Debug)]
struct MatchedLine {
    line: LogLine,
    rule: &Rule,
    fields: Option<HashMap<String,String>>,
}²

impl MatchedLine {
    fn new(line: LogLine, rule: &Rule) -> MatchedLine {
        MatchedLine{
            line: line,
            rule: rule,
            fields: None,
        }
    }
}
*/



fn main() {
    let opt = Opt::from_args();
    let settings = settings::Settings::load(opt.config.unwrap_or(PathBuf::from("/etc/timber/timber.conf")));

    let mut rules = HashMap::new();
    rules.insert(String::from("sshd"),Vec::new());
    rules.get_mut("sshd").unwrap().push(matcher::Rule::new("ssh-accepted","sshd",r"Accepted"));
    rules.get_mut("sshd").unwrap().push(matcher::Rule::new("ssh-disconnect","sshd",r"Disconnect"));
    rules.get_mut("sshd").unwrap().push(matcher::Rule::new("ssh-deny","sshd",r"Failed password for invalid user (?P<username>.+) from (?P<src_ip>.+) port"));

    loop {
        let line = match parser::read_line(){
            Some(line) => line,
            None => break
        }; 
        let logline = match parser::LogLine::from_string(&line,&settings){
            Ok(logline) => logline,
            Err(_) => continue,
        };
        let process_rules = match rules.get(logline.process()){
            Some(v) => v,
            None => continue,
        };
        for rule in process_rules {
            if rule.is_match(&logline){
                println!(">Rule {} matched : \n\t{:?}",rule.name(),logline.raw())
                //let matched_line = MatchedLine::new(logline, &rule);
                //println!("{:?}", matched_line);
            }
        }

    }
}
