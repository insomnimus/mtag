mod app;
mod cmd;
mod set;

fn main() {
	std::process::exit(match cmd::run() {
		Err(e) => {
			eprintln!("error: {}", e);
			1
		}
		Ok(code) => code,
	});
}
