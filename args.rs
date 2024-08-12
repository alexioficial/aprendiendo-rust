use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("Argumento 1: {}", args[1]);
}