use std::io::{self, Write};

pub fn write(value: &str) {
    println!("{}", value);
}

pub fn ask(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = io::stdout().flush();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}
