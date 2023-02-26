use std::{env, process};
use minigrep::{Config, run};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Se for {}", &config.query);
    println!("In file {}", &config.file_path);

    if let Err(err) = run(config) {
        eprint!("Errrrrrr, e: {err}");
        process::exit(1)
    }
    
}

