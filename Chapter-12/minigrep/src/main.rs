use minigrep::{run, Config};
use std::{env, process};

// Making a grep program but implemented in rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        eprintln!("mg: {}: {}", &config.path, e);
        process::exit(1)
    }
}