use std::io;

fn main() {
    let mut lines: Vec<String> = vec![];
    loop {
        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => lines.push(line),
            Err(error) => {
                println!("error: {}", error);
                ::std::process::exit(1);
            }
        }
    }
    lines.sort();
    for line in lines {
        print!("{}", line);
    }
}
