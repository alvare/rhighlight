extern crate regex;
extern crate clap;
extern crate colored;

use std::io;
use std::io::{Write, BufWriter};
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
    let re = Regex::new(pattern).unwrap();

    let mut input = String::new();
    let mut stream = BufWriter::new(io::stdout());

    while io::stdin().read_line(&mut input).unwrap() > 0 {
        {
            let result = re.replace_all(input.as_str(), |cap: &Captures| {
                cap[0].red().to_string()
            });

            stream.write(&result.as_bytes()).unwrap();
        }
        input.clear();
    }
}
