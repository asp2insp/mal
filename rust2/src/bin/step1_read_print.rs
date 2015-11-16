extern crate regex;

use regex::Regex;
use std::io::{self, Write};

fn read(input: String) -> MalType {
    Reader::read_str(input)
}

fn eval(ast: MalType) -> MalType {
    ast
}

fn print(exp: MalType) -> String {
    print_str(exp)
}

fn rep(input: String) -> String {
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

enum MalType {
    List(Vec<MalType>),
    Int(i32),
    Nil,
    True,
    False,
    Str(String),
    Sym(String),
    Error,
}

struct Reader{
    tokens: Vec<String>,
    pos: usize,
}

impl Reader {
    fn read_str(input: String) -> MalType {
        let r = Reader {
            tokens: Reader::tokenizer(input),
            pos: 0,
        };
        r.read_form()
    }

    fn tokenizer(input: String) -> Vec<String> {
        let re = Regex::new(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#)
            .unwrap();
        re.split(input).collect()
    }

    fn next(&mut self) -> Option<String> {
        self.pos += 1;
        self.tokens.get(self.pos-1)
    }

    fn peek(&self) -> Option<String> {
        self.tokens.get(self.pos)
    }

    fn read_form(&mut self) -> MalType {
        match self.peek() {
            Some("(") => self.read_list(),
            Some(_)   => self.read_atom(),
            None => MalType::Error,
        }
    }

    fn read_list(&mut self) -> MalType {
        let ret : Vec<MalType> = Vec::new();
        loop {
            match self.next() {
                Some(")") => break,
                Some(_)  => ret.push(self.read_form()),
                None => ret.push(MalType::Error),
            }
        }
        MalType::List(ret)
    }

    fn read_atom(&mut self) -> MalType {
        match self.next() {
            Some("nil") => MalType::Nil,
            Some("true") => MalType::True,
            Some("false") => MalType::False,
            None => MalType::Error,
        }
    }
}

pub fn print_str(ast: &MalType) -> String {
    match ast {
        &MalType::List(l) => {
            format!("({})", l.iter().map(|&e| print_str(&e)).join(" ")).as_str()
        },
        &MalType::Int(i) => format!("{}", i).as_str(),
        &MalType::Nil => "nil",
        &MalType::True => "true",
        &MalType::False => "false",
        &MalType::Str(s) => format!("{}", s).as_str(),
        &MalType::Sym(s) => format!("{}", s).as_str(),
    }
}
