pub struct Clear {
	files: Vec<String>,
	all: bool,
	title: bool,
	album: bool,
	genre: bool,
	category: bool,
	desc: bool,
	media_type: bool,
	artwork: bool,
}

pub fn clear(m: &ArgMatches) -> i32 {
	macro_rules! clear {
		($name:literal, $method:ident) => {
			if all || m.is_present($name) {
				tag.$method();
			}
		};
	}
	
	let mut n_err = 0;
	let all = m.is_present("keys");
	for tag in m.values_of("file")
	.unwrap()
	.map(|s| Tag::read_from_path(s))
	{
		let mut tag = match tag {
			Ok(t) => t,
			Err(e) => {
				n_err += 1;
				eprintln!("error: {}", e);
				continue;
			};
			clear!("artist", remove_artists);
			clear!("genre", remove_genres)
			clear!("track", remove_track),
			clear!("disc", remove_disc),
			clear!("album", remove_album),
			clear!("copyright", remove_copyright),
			clear!("lyrics", )
		}
	}
}