use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    if args.len() < 2 {
        println!("Usage: {} <input>", program);
        return;
    }

    let echo_arg = args[1].clone();
    println!("{}", echo_arg);
}