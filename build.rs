use std::{
	env,
	io::Error,
};

use clap_complete::{
	generate_to,
	shells::{
		Bash,
		Elvish,
		Fish,
		PowerShell,
		Zsh,
	},
};

include!("src/app.rs");

fn main() -> Result<(), Error> {
	let out_dir = match env::var("OUT_DIR") {
		Err(_) => return Ok(()),
		Ok(s) => s,
	};
	let mut app = new();

	macro_rules! gen {
		($sh:expr) => {
			generate_to($sh, &mut app, "mtag", &out_dir)
		};
	}

	gen!(PowerShell)?;
	gen!(Bash)?;
	gen!(Elvish)?;
	gen!(Zsh)?;
	gen!(Fish)?;

	Ok(())
}
