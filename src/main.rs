extern crate regex;
extern crate clap;
extern crate colored;

use std::io;
use std::io::Write;
use regex::{Regex, Captures};
use clap::{Arg, App};
use colored::*;



fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("PATTERN")
            .help("Regular expression to higlight")
            .required(true))
        .get_matches();

    let pattern = matches.value_of("PATTERN").unwrap();
    //let re = Regex::new(pattern).unwrap();

    let mut input = String::new();
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //let result = re.replace_all(input.as_str(), |cap: &Captures| {
                //    cap[0].red().to_string()
                //});
                let result = input.replace(pattern, &pattern.red().to_string());

                io::stdout().write(result.as_bytes()).unwrap();

                if input.is_empty() { break }
            }
            Err(error) => println!("IO error: {}", error),
        }
    }
}
