use anyhow::Result;
use clap::ArgMatches;
use mp4ameta::Tag;

use crate::{
	app,
	set,
};

fn run_get(m: &ArgMatches) -> i32 {
	let mut n_err = 0;
	for p in m.values_of("file").unwrap() {
		match Tag::read_from_path(p) {
			Ok(t) => println!("# {}:\n{}\n", p, &t),
			Err(e) => {
				n_err += 1;
				eprintln!("error reading {}: {}", p, e);
			}
		}
	}
	n_err
}

fn run_clear(m: &ArgMatches) -> i32 {
	let mut n_err = 0;
	for p in m.values_of("file").unwrap() {
		match Tag::read_from_path(p) {
			Err(e) => {
				eprintln!("error reading {}: {}", p, e);
				n_err += 1;
			}
			Ok(mut tag) => {
				tag.clear();
				match tag.write_to_path(p) {
					Err(e) => {
						eprintln!("error writing {}: {}", p, e);
						n_err += 1;
					}
					Ok(_) => println!("cleared: {}", p),
				}
			}
		}
	}
	n_err
}

pub fn run() -> Result<i32> {
	let m = app::new().get_matches_from(wild::args());
	match m.subcommand().unwrap() {
		("get", m) => Ok(run_get(m)),
		("clear", m) => Ok(run_clear(m)),
		("set", m) => set::run(m),
		_ => panic!("unreachable code"),
	}
}
