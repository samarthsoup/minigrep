use std::env;
use std::process;

use minigrep::Config; //adding fns/structs in lib.rs means you call it like this
//treat the lib.rs file here as your own defined crate

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}"); //if build() returns an err, it prints here
        process::exit(1);
    }); //build will unwrap, if it returns Ok() it does its function, and if it returns an Err it writes the error to stderr

    if let Err(e) = minigrep::run(config) { //any file read errors, or converting file contents to string errors are handled by the if let Err
        eprintln!("application error: {e}");
        process::exit(1);
    }
}