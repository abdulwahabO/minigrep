use std::env;
use std::process;

use minigrep::Config;

fn main() {
    
    // Collect command line arguments passed to the program into a vector of strings.
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Could not perform search: {}", err);
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config) {
        eprintln!("FAILED: {}", err);
        process::exit(1);
    };
}
