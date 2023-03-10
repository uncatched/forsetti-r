use std::{env, process};
use forsetti_r::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{:?}", config);
}