use std::{
	fs,
	path::Path,
};

use anyhow::{
	anyhow,
	Result,
};
use clap::ArgMatches;
use mp4ameta::{
	Img,
	ImgFmt,
	MediaType,
	Tag,
};

fn read_img(p: impl AsRef<Path>) -> Result<Img<Vec<u8>>> {
	let ext = p
		.as_ref()
		.extension()
		.ok_or_else(|| anyhow!("image format can't be determined: {}", p.as_ref().display()))?;

	let fmt = if ext.eq_ignore_ascii_case("png") {
		ImgFmt::Png
	} else if ext.eq_ignore_ascii_case("bmp") {
		ImgFmt::Bmp
	} else if ext.eq_ignore_ascii_case("jpeg") || ext.eq_ignore_ascii_case("jpg") {
		ImgFmt::Jpeg
	} else {
		anyhow::bail!("unsupported image extension: {}", ext.to_string_lossy());
	};

	let data = fs::read(p.as_ref())?;
	Ok(Img { fmt, data })
}

fn parse_media_type(s: &str) -> Option<MediaType> {
	use MediaType::*;
	Some(match &s.to_lowercase()[..] {
		"movie" => Movie,
		"normal" => Normal,
		"audiobook" => AudioBook,
		"music-video" => MusicVideo,
		"short-film" => ShortFilm,
		"tv-show" => TvShow,
		"booklet" => Booklet,
		_ => return None,
	})
}

fn parse_track(s: &str) -> Option<(u16, u16)> {
	s.split_once('/')
		.map(|(left, right)| (left.parse::<u16>().unwrap(), right.parse::<u16>().unwrap()))
}

pub fn run(m: &ArgMatches) -> Result<i32> {
	macro_rules! arg {
		($name:literal) => {
			m.value_of($name)
				.map(|s| if s.is_empty() { None } else { Some(s) })
		};
	}
	macro_rules! args {
		($name:literal) => {
			m.values_of($name).map(|it| {
				let xs = it.map(String::from).collect::<Vec<_>>();
				if xs.is_empty() || (xs.len() == 1 && xs[0].is_empty()) {
					None
				} else {
					Some(xs)
				}
			})
		};
	}

	let artists = args!("artist");
	let title = arg!("title");
	let album = arg!("album");
	let genres = args!("genre");
	let categories = args!("category");
	let description = arg!("description");
	let media_type = m.value_of("type").map(parse_media_type);
	let artwork = arg!("artwork");
	let bpm = m.value_of("bpm").map(|s| s.parse::<u16>().ok());
	let track = m.value_of("track").map(parse_track);
	let disc = m.value_of("disc").map(parse_track);
	let isrc = arg!("isrc");
	let show = arg!("show");
	let work = arg!("work");
	let year = arg!("year");
	let copyright = arg!("copyright");

	let img = match artwork {
		None => None,
		Some(None) => Some(None),
		Some(Some(path)) => Some(Some(read_img(path)?)),
	};

	let mut n_err = 0;
	for p in m.values_of("file").unwrap() {
		let mut tag = match Tag::read_from_path(p) {
			Err(e) => {
				n_err += 1;
				eprintln!("error reading metadata of {}: {}", p, e);
				continue;
			}
			Ok(x) => x,
		};

		macro_rules! set {
			($val:ident, $set:ident, $remove:ident) => {
				match &$val {
					None => (),
					Some(None) => tag.$remove(),
					Some(Some(val)) => tag.$set(val.clone()),
				}
			};
		}

		match &img {
			None => (),
			Some(None) => tag.remove_artworks(),
			Some(Some(img)) => tag.set_artwork(img.clone()),
		}

		set!(artists, set_artists, remove_artists);
		set!(title, set_title, remove_title);
		set!(album, set_album, remove_album);
		set!(genres, set_genres, remove_genres);
		set!(categories, set_categories, remove_categories);
		set!(description, set_description, remove_descriptions);
		set!(media_type, set_media_type, remove_media_type);
		set!(bpm, set_bpm, remove_bpm);
		match track {
			Some(None) => tag.remove_track(),
			Some(Some((n, total))) => tag.set_track(n, total),
			None => (),
		};
		match disc {
			Some(None) => tag.remove_disc(),
			Some(Some((n, total))) => tag.set_disc(n, total),
			None => (),
		};
		set!(copyright, set_copyright, remove_copyright);
		set!(isrc, set_isrc, remove_isrc);
		set!(show, set_tv_show_name, remove_tv_show_name);
		set!(work, set_work, remove_work);
		set!(year, set_year, remove_year);

		match tag.write_to_path(p) {
			Err(e) => {
				eprintln!("error writing {}: {}", p, e);
				n_err += 1;
			}
			Ok(_) => {
				println!("tagged: {}", p);
			}
		}
	}

	Ok(n_err)
}
