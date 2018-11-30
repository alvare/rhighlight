extern crate regex;
extern crate clap;
extern crate colored;

use std::io;
use std::io::{Write, BufRead};
use regex::bytes::{Regex, Captures};
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
    let blah = pattern.red().to_string();
    let replacement = blah.as_bytes();

    let mut input = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    while handle.read_until(b'\n', &mut input).unwrap() > 0 {
        {
            //let result = re.replace_all(input.as_str(), |cap: &Captures| {
            //    cap[0].red().to_string()
            //});
            let matches = re.find_iter(&input);
            let mut output = Vec::with_capacity(input.len() + input.len()/4);
            let mut last_match = 0;
            for mat in matches {
                output.extend_from_slice(&input[last_match..mat.start()]);
                output.extend_from_slice(&replacement);
                last_match = mat.end();
            }
            output.extend_from_slice(&input[last_match..]);

            io::stdout().write(&output).unwrap();
        }
        input.clear();
    }
}
