use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_config(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {:?}", err);
        process::exit(1);
    });
    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.filename);
    // Need to check for errors here when calling the run() function
    //
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {:?}", e);
        process::exit(1);
    }
}
