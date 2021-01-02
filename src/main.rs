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
    #[structopt(parse(from_os_str), short, long)]
    rulefile: Option<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    //Load settings
    let settings = settings::Settings::load(opt.config.unwrap_or(PathBuf::from("/etc/timber/config.toml")));
    //Load rules
    let rules = matcher::Rule::load(opt.rulefile.unwrap_or(PathBuf::from("/etc/timber/rules.timber")));

    loop {
        //Read a log line
        let line = match parser::read_line(){
            Some(line) => line,
            None => break,
        }; 
        //Parse the log line
        let logline = match parser::LogLine::from_string(&line,&settings){
            Ok(logline) => Rc::new(logline),
            Err(_) => continue,
        };
        //Get rules for log line process
        let process_rules = match rules.get(logline.process()){
            Some(rules_vect) => rules_vect,
            None => continue,
        };
        //For each rule in log process, see if line matches
        for rule in process_rules {
            let _matched_line = match rule.is_match(&logline){
                Some(line) => line,
                None => continue,
            };
        }

    }
}
