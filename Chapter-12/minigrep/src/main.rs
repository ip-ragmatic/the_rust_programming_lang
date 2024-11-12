use minigrep::{run, Config};
use std::{env, process};

// // Making a grep program but implemented in rust
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let config = Config::build(&args).unwrap_or_else(|err| {
//         eprintln!("{}", err);
//         process::exit(1)
//     });
//
//     if let Err(e) = run(&config) {
//         eprintln!("mg: {}: {}", &config.path, e);
//         process::exit(1)
//     }
// }

// updated version from section 13.3. It's commented out because I haven't figured out how to
// implement flags using the iterator input.
fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1)
    });

    if let Err(e) = run(&config) {
        eprintln!("mg: {}: {}", &config.path, e);
        process::exit(1)
    }
}

