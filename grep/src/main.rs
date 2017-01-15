extern crate getopts;
extern crate regex;

use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

use getopts::Options;
use regex::Regex;

fn do_work(
    pattern: &String,
    filenames: &Vec<String>,
    show_number: bool
    )
{
    let re = Regex::new(pattern).unwrap();

    for filename in filenames {
        let content = match read_file(filename) {
            Ok(content) => content,
            Err(reason) => panic!(reason)
        };
        for (index, line) in content.lines().enumerate() {
            if re.is_match(line) {
                print_line(line, index + 1, show_number);
            };
        }
    }
}

fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file    = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn print_line(line: &str, number: usize, show_number: bool) {
    if show_number {
        print!("{:6}  ", number);
    }
    println!("{}", line);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("n", "number", "show line number");

	let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let pattern     = matches.free[0].clone();
    let filenames   = matches.free[1..].to_vec();
    let show_number = matches.opt_present("n");

    do_work(&pattern, &filenames, show_number);
}
