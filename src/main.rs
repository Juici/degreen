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

    if let Some(shell) = app.completion_shell() {
        App::gen_completions(shell);
        return Ok(true);
    }

    let settings = app.settings();

    // TODO: actual functionality

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
