use std::io;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lines: Vec<String>;
    if args.len() > 1 {
        let filename = &(args[1]);
        lines = lines_from_file(filename);
    } else {
        lines = lines_from_stdin();
    }
    lines.sort();
    for line in lines {
        if line.ends_with("\n") {
            print!("{}", line);
        } else {
            println!("{}", line);
        }
    }
}

fn lines_from_stdin() -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    loop {
        let mut line: String = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => lines.push(line),
            Err(error) => {
                println!("error: {}", error);
                ::std::process::exit(1);
            }
        }
    }
    lines
}

fn lines_from_file(filename: &str) -> Vec<String> {
    match fs::read_to_string(filename) {
        Ok(contents) => {
            return contents.lines().map(|x| x.to_owned()).collect()
        }
        Err(error) => {
            println!("error: {}", error);
            ::std::process::exit(1);
        }
    }
    
}