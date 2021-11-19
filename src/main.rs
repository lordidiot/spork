//use std::io;
use std::env;
use std::process;
use spork::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Argument error: {}", err);
        process::exit(1);
    });

    if let Err(error) = spork::run(config){
        println!("Application error: {}", error);
        process::exit(1);
    }
}