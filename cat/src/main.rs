use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <input>", program);
        return;
    }

    let input = args[1].clone();
    
    let mut f = match File::open(&input) {
        Err(why) => panic!("couldn't open {}: {}", input, why),
        Ok(file) => file,
    };
    
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    println!("{}", contents);
}
