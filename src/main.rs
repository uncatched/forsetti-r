use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    println!("{:?}", config);
}

#[derive(Debug)]
pub struct Config {
    pub bundle_id: String,
    pub device_token: String,
    pub team_id: String,
    pub key_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 5 {
            return Err("Not enough arguments.")
        }

        let bundle_id = args[1].clone();
        let device_token = args[2].clone();
        let team_id = args[3].clone();
        let key_path = args[4].clone();

        Ok(
            Config { 
                bundle_id, 
                device_token, 
                team_id, 
                key_path 
            }
        )
    }
}