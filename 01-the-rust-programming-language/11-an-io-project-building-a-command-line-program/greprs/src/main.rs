extern crate greprs;

use std::env;
use std::process;

use greprs::Config;

fn main() {
    //== Terminal args =========================================================
    //-- Get terminal args -----------------------------------------------------
    let args: Vec<String> = env::args().collect();

    // https://stackoverflow.com/questions/39204908/how-to-check-release-debug-builds-using-cfg-in-rust
    #[cfg(debug_assertions)] // for debug mode
    println!("{:?}\n", args);

    //-- Each args -------------------------------------------------------------
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    #[cfg(debug_assertions)]
    {
        println!("Searching for {}", config.query);
        println!("In file {}\n",  config.filename);
    }

    //== File ==================================================================
    if let Err(e) = greprs::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
