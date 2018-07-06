extern crate clap;
#[macro_use]
extern crate pkg;

mod error;

use std::process;

use error::{Error, Result};

fn run() -> Result<bool> {
    Ok(true)
}

fn main() {
    let result = run();

    match result {
        Err(error) => {
            eprintln!("error: {}", error);
            process::exit(1);
        }
        Ok(false) => process::exit(1),
        Ok(true) => process::exit(0),
    }
}
