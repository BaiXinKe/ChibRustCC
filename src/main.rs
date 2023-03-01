use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments\n", args[0]);
        exit(1)
    }

    let p = &args[1];

    println!(" .global main");
    println!("main:");

    if let Some(first_op_index) = p.find(|x| x == '+' || x == '-') {
        println!(" mov ${}, %rax", p[0..first_op_index].to_owned());
        let res = p[first_op_index..].chars().collect::<Vec<_>>();

        let mut index = 0usize;
        while index < res.len() {
            match res[index] {
                '+' | '-' => {
                    let num = res[index + 1..]
                        .iter()
                        .take_while(|&&c| c.is_ascii_digit())
                        .collect::<String>();

                    match res[index] {
                        '+' => println!(" add ${num}, %rax"),
                        '-' => println!(" sub ${num}, %rax"),
                        _ => unreachable!(),
                    }
                    index += num.len() + 1;
                }
                '0'..='9' => {
                    index += 1;
                    continue;
                }
                _ => {
                    panic!("unexpected character: {}", res[index])
                }
            }
        }
    } else {
        println!(" mov ${}, %rax", args[1]);
    }

    println!(" ret");
}
