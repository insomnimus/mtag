use mtag::cmd;
use std::process;

fn main() {
    if let Some(e) = cmd::run() {
        eprintln!("error: {}", &e);
        process::exit(1);
    }
}
