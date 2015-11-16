use std::io::{self, Write};

pub fn read(input: &str) -> &str {
    input
}

pub fn eval(ast: &str) -> &str {
    ast
}

pub fn print(exp: &str) -> &str {
    exp
}

pub fn rep(input: &str) -> &str {
    return print(eval(read(input)));
}

pub fn main() {
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");
        println!("{}", rep(input.trim()));
    }
}
