#![allow(unused_mut, unused_variables)]

use mod1::Config;

fn main() {
    let config = Config::new(std::env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = mod1::run(config) {
        eprintln!("Application error: {}", e);

        std::process::exit(1);
    }
}
