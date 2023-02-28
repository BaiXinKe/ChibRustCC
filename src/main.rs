use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments\n", args[0]);
        exit(1)
    }

    println!(" .global main");
    println!("main:");
    println!(" mov ${}, %rax", args[1]);
    println!(" ret");
}
