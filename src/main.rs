use std::io::{self, Read};
use std::fs;

use structopt::StructOpt;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
#[structopt(name="rort",about="Reimplementation of /usr/bin/sort")]
struct Opt {
    #[structopt(short="v", long="version")]
    show_version: bool,
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    if opt.show_version {
        println!("0.1");
        return;
    }
    let data = fetch_data(&opt);
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

fn fetch_data(opt: &Opt) -> String {
    if opt.files.len() > 0 {
        let filename = &(opt.files[0]);
        read_from_file(filename)
    } else {
        read_from_stdin()
    }
}

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer).expect("couldn't read from stdin");
    buffer
}

fn read_from_file(filename: &PathBuf) -> String {
    fs::read_to_string(filename).expect("couldn't read from file")
}