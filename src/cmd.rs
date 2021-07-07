use crate::app;
use clap::ArgMatches;
use mp4ameta::{Error, Img, ImgFmt, MediaType, Tag};
use std::{fs, path::Path};

struct ClearCmd {
    files: Vec<String>,
    clear_artwork: bool,
}

impl ClearCmd {
    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            files: m.values_of("file").unwrap().map(String::from).collect(),
            clear_artwork: m.is_present("artwork"),
        }
    }

    fn run(&self) -> Result<(), Error> {
        for f in &self.files {
            let mut tag = Tag::read_from_path(f)?;
            println!("clearing metadata from {}", f);

            if self.clear_artwork {
                tag.remove_artworks();
            }
            tag.remove_artists();
            tag.remove_genres();
            tag.remove_track();
            tag.remove_disc();
            tag.remove_album();
            tag.remove_copyright();
            tag.remove_lyrics();
            tag.remove_movement();
            tag.remove_title();
            tag.remove_tv_episode_name();
            tag.remove_tv_network_name();
            tag.remove_tv_show_name();
            tag.remove_work();
            tag.remove_year();
            tag.remove_isrc();
            tag.remove_album_artists();
            tag.remove_categories();
            tag.remove_comments();
            tag.remove_composers();
            tag.remove_custom_genres();
            tag.remove_descriptions();
            tag.remove_groupings();
            tag.remove_keywords();
            tag.remove_lyricists();
            tag.remove_show_movement();
            tag.remove_movement_count();
            tag.remove_movement_index();
            tag.remove_tv_episode();
            tag.remove_tv_season();
            tag.remove_artworks();
            tag.remove_advisory_rating();

            tag.write_to_path(f)?;
            println!("cleared all metadata from {}", f);
        }

        Ok(())
    }
}

struct GetCmd {
    files: Vec<String>,
}

impl GetCmd {
    fn from_matches(m: &ArgMatches) -> Self {
        Self {
            files: m.values_of("file").unwrap().map(String::from).collect(),
        }
    }

    fn run(&self) -> Result<(), Error> {
        for f in &self.files {
            let tag = Tag::read_from_path(f)?;
            println!("#{}:\n{}", f, &tag);
        }
        Ok(())
    }
}

struct SetCmd {
    files: Vec<String>,
    title: Option<String>,
    artists: Option<Vec<String>>,
    album: Option<String>,
    genres: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    media_type: Option<MediaType>,
    description: Option<String>,
    artwork: Option<String>,
}

impl SetCmd {
    fn from_matches(m: &ArgMatches) -> Self {
        let files: Vec<_> = m.values_of("file").unwrap().map(String::from).collect();

        let artwork = m.value_of("artwork").map(String::from);
        let title = m.value_of("title").map(String::from);
        let artists = m
            .values_of("artist")
            .map(|i| i.map(String::from).collect::<Vec<_>>());

        let album = m.value_of("album").map(String::from);

        let genres = m
            .values_of("genre")
            .map(|i| i.map(String::from).collect::<Vec<_>>());

        let categories = m
            .values_of("category")
            .map(|i| i.map(String::from).collect::<Vec<_>>());

        let media_type = m.value_of("type").map(match_type);
        let description = m.value_of("description").map(String::from);

        Self {
            files,
            title,
            artists,
            album,
            genres,
            categories,
            description,
            media_type,
            artwork,
        }
    }

    fn run(&self) -> Result<(), Error> {
        for f in &self.files {
            let mut tag = Tag::read_from_path(f)?;
            println!("tagging {}", f);

            if let Some(i) = self.artwork.as_ref() {
                if i.is_empty() {
                    tag.remove_artworks();
                } else {
                    // this is acceptable in a loop
                    // because 99% of the time, there's only 1 file
                    let img = read_img(i)?;
                    tag.set_artwork(img);
                }
            }

            if let Some(t) = self.title.as_ref() {
                if t.is_empty() {
                    tag.remove_title();
                } else {
                    tag.set_title(t);
                }
            }

            if let Some(a) = self.artists.as_ref() {
                tag.remove_composers();
                tag.remove_artists();
                tag.remove_album_artists();
                for art in a {
                    tag.add_artist(art);
                    tag.add_album_artist(art);
                    tag.add_composer(art);
                }
            }

            if let Some(a) = self.album.as_ref() {
                if a.is_empty() {
                    tag.remove_album();
                } else {
                    tag.set_album(a);
                }
            }

            if let Some(g) = self.genres.as_ref() {
                tag.remove_genres();
                tag.remove_custom_genres();
                for gen in g {
                    tag.add_custom_genre(gen);
                    // tag.add_genre(gen);
                }
            }

            if let Some(c) = self.categories.as_ref() {
                tag.remove_categories();
                for cat in c {
                    tag.add_category(cat);
                }
            }

            if let Some(t) = self.media_type.as_ref() {
                tag.set_media_type(*t);
            }

            if let Some(d) = self.description.as_ref() {
                if d.is_empty() {
                    tag.remove_descriptions();
                } else {
                    tag.set_description(d);
                }
            }

            tag.write_to_path(f)?;
            println!("successfully tagged {}", f);
        }

        Ok(())
    }
}

fn match_type(s: impl AsRef<str>) -> MediaType {
    use MediaType::*;
    match &s.as_ref().to_lowercase()[..] {
        "movie" => Movie,
        "normal" => Normal,
        "audiobook" => AudioBook,
        "music-video" => MusicVideo,
        "short-film" => ShortFilm,
        "tv-show" => TvShow,
        "booklet" => Booklet,
        _ => unreachable!(),
    }
}

pub fn run() -> Result<(), Error> {
    let m = app::new().get_matches();
    let cmd = m.subcommand_name().unwrap();
    let cmd_matches = m.subcommand_matches(cmd).unwrap();
    match cmd {
        "set" => SetCmd::from_matches(cmd_matches).run(),
        "get" => GetCmd::from_matches(cmd_matches).run(),
        "clear" => ClearCmd::from_matches(cmd_matches).run(),
        _ => unreachable!(),
    }
}

fn read_img(p: impl AsRef<Path>) -> Result<Img<Vec<u8>>, Error> {
    let ext = p
        .as_ref()
        .extension()
        .map(|s| {
            s.to_str()
                .map(str::to_lowercase)
                .unwrap_or_else(String::new)
        })
        .unwrap_or_default();

    let fmt = match &ext[..] {
        "png" => ImgFmt::Png,
        "jpeg" => ImgFmt::Jpeg,
        "bmp" => ImgFmt::Bmp,
        _ => {
            eprintln!("{}: invalid image format", p.as_ref().display());
            std::process::exit(2);
        }
    };

    let data = fs::read(p.as_ref())?;
    Ok(Img { fmt, data })
}
