use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "exit 0" {break}
        let words:  Vec<&str> = input.split_whitespace().collect();

        match words.split_first() {
            Some((&"echo", rest)) => {
                let output = rest.join(" ");
                println!("{}", output);
            }
            Some((cmd, _)) => {
                println!("novus-shell: Command not found: {}", cmd);
            }
            None => {
                continue
            }
        }
    }
}


