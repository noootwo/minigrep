use std::process;

use minigrep::Config;

fn main() {
    let config = Config::build().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        process::exit(1);
    });
}
