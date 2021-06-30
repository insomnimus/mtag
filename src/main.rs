use mtag::cmd;
use std::process;

fn main() {
    if let Err(e) = cmd::run() {
        eprintln!("error: {}", &e);
        process::exit(1);
    }
}
