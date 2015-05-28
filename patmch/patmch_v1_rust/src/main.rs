extern crate regex;

use std::env;
use std::io;
use std::io::BufRead;
use std::io::Write;
use regex::Regex;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        writeln!(io::stderr(), "Usage: {} regexp < in.file\n", args[0])
            .unwrap();
    }

    let re = Regex::new(&args[1]).ok().expect("Failed to parse regex");

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        // TODO: This expects inputs to be in utf-8. howto.bz2 is latin1.
        let line = line.ok().expect("Failed to read from stdin");
        if re.is_match(&line) {
            print!("{}", line);
        }
    }
}
