use std::env;

fn main() {
    println!("--- Palmera CLI ---");
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", args[1]);
    }
}
