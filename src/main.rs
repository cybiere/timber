use std::collections::HashMap;
use structopt::StructOpt;
use std::path::PathBuf;
use std::rc::Rc;

mod settings;
mod parser;
mod matcher;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str), short, long)]
    config: Option<PathBuf>,
}

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
            Ok(logline) => Rc::new(logline),
            Err(_) => continue,
        };
        let process_rules = match rules.get(logline.process()){
            Some(v) => v,
            None => continue,
        };
        for rule in process_rules {
            let _matched_line = match rule.is_match(&logline){
                Some(line) => line,
                None => continue,
            };
        }

    }
}
