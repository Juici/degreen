extern crate clap;
#[macro_use]
extern crate pkg;

mod app;
mod error;

use std::process;

use app::App;
use error::Result;

fn run() -> Result<bool> {
    let app = App::new();
    let matches = app.get_matches();

    if let Some(shell) = matches.value_of("completions") {
        App::new().gen_completions(shell.parse().unwrap());
        return Ok(true);
    }

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
