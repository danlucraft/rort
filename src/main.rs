use std::io::{self, Read};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let data: String;
    if args.len() > 1 {
        let filename = &(args[1]);
        data = read_from_file(filename);
    } else {
        data = read_from_stdin();
    }
    let mut lines: Vec<&str> = data.lines().collect();
    lines.sort();
    for line in lines {
        if line.ends_with("\n") {
            print!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).expect("couldn't read from stdin");
    buffer
}

fn read_from_file(filename: &str) -> String {
    fs::read_to_string(filename).expect("couldn't read from file")
}