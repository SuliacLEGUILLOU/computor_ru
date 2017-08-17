
extern crate regex;

pub mod history;
pub mod lexer;
pub mod parser;
pub mod my_math;
pub mod message;
pub mod stack;

use std::io;

use history::history::History;

use parser::parser::*;
use lexer::lexer::*;

use my_math::num_complexe::num_complexe::NumComplexe;
//use my_math::array::array::Array;
use message::information::*;
use message::error::*;
use stack::stack::Stack;

fn main() {
    let mut memory: Stack = Stack::new();
    let mut history: History = History::new();

    welcome();

    parse_var("90");

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let line = input.trim();
        history.add(line);

        match line {
            "exit" => { break },
            "e" => { break },
            "help" => { help(); continue },
            "h" => { help(); continue },
            "print" => { memory.print_all(); continue },
            "p" => { memory.print_all(); continue },
            "pv" => { memory.print_variable(); continue },
            "print_var" => { memory.print_variable(); continue },
            _ => { no_input() }
        }

        parse_cmd(check_input(line));
    }

    memory.clone().print_all();
    history.print();
}
