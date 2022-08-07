extern crate grep_rust;
use grep_rust::{run, Config};
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("search ing for {}", config.query);
    println!("In file {}", config.filename);
    if let Err(e) = run(config) {
        println!("Application error:{}", e);
        process::exit(1);
    }
}
