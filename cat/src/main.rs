use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn read_file(filename: &String) -> Result<String, io::Error> {
    let mut file    = try!(File::open(filename));
    let mut content = String::new();
    try!(file.read_to_string(&mut content));
    Ok(content)
}

fn show_content(content: &String, show_numbers: bool) {
    let lines = content.split("\n");
    for (index, line) in lines.enumerate() {
        if show_numbers {
            print!("{:6}  ", index)
        }
        println!("{}", line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut show_numbers = false;
    for arg in args.iter().skip(1) {
        match arg.as_ref() {
            "-n" => {
                show_numbers = true
            },
            _ => {
                match read_file(arg) {
                    Ok(content) => show_content(&content, show_numbers),
                    Err(reason) => panic!(reason)
                };
            },
        };
    }
}
