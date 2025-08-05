use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if let Err(e) = rep::run(args) {
        eprintln!("{}", e);
        eprintln!();
        eprintln!("💭 Need help? Run with --help for usage information");
        process::exit(1);
    }
}


