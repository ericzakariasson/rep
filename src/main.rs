use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(e) = rep::run(args) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}


