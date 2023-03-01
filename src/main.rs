use crate::token::tokenize;
use std::process::exit;
mod token;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments\n", args[0]);
        exit(1)
    }

    let tokens = tokenize(&args[1]).expect("Unexpected error happend");

    println!(" .global main");
    println!("main:");

    let mut tokens_iter = tokens.iter();

    println!(" mov ${}, %rax", tokens_iter.next().unwrap().val.unwrap());

    while let Some(tok) = tokens_iter.next() {
        if tok.is_plus() {
            println!(" add ${}, %rax", tokens_iter.next().unwrap().val.unwrap());
            continue;
        }
        println!(" sub ${}, %rax", tokens_iter.next().unwrap().val.unwrap());
    }

    println!(" ret");
}
