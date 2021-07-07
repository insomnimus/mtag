use clap::{
    crate_version, App, AppSettings, Arg, ArgGroup,
    ArgSettings::{AllowEmptyValues, IgnoreCase, UseValueDelimiter},
};

pub fn new() -> App<'static> {
    App::new("mtag")
        .version(crate_version!())
        .about("Edit mpeg-4 metadata.")
        .setting(AppSettings::SubcommandRequired)
        .global_setting(AppSettings::VersionlessSubcommands)
        .global_setting(AppSettings::UnifiedHelpMessage)
        .subcommand(app_clear())
        .subcommand(app_set())
        .subcommand(app_get())
}

fn app_clear() -> App<'static> {
    App::new("clear")
        .visible_alias("c")
        .aliases(&["clean", "purge"])
        .about("Clear all metadata.")
        .arg(
            Arg::new("artwork")
                .long("artwork")
                .visible_alias("art")
                .about("Clear media artwork as well."),
        )
        .arg(
            Arg::new("file")
                .multiple(true)
                .about("file to clear the metadata of")
                .required(true),
        )
}

fn app_set() -> App<'static> {
    const MEDIA_TYPES: &[&str] = &[
        "movie",
        "normal",
        "audiobook",
        "music-video",
        "short-film",
        "tv-show",
        "booklet",
    ];

    let app = App::new("set")
		.visible_alias("s")
		.about("Set media metadata.")
		.after_long_help("To clear a particular key, you can pass empty values to any argument, for example `--genre=''`");

    let artist = Arg::new("artist")
        .long("artist")
        .takes_value(true)
        .setting(UseValueDelimiter)
        .setting(AllowEmptyValues)
        .about("comma separated list of artists to set");

    let title = Arg::new("title")
        .long("title")
        .short('t')
        .alias("tit")
        .visible_alias("ttl")
        .takes_value(true)
        .setting(AllowEmptyValues)
        .about("set the title metadata");

    let album = Arg::new("album")
        .long("album")
        .visible_alias("alb")
        .takes_value(true)
        .setting(AllowEmptyValues)
        .about("set the album metadata");

    let genre = Arg::new("genre")
        .long("genre")
        .visible_alias("gen")
        .short_alias('g')
        .takes_value(true)
        .setting(UseValueDelimiter)
        .setting(AllowEmptyValues)
        .about("comma separated list of genres to set");

    let category = Arg::new("category")
        .takes_value(true)
        .setting(UseValueDelimiter)
        .setting(AllowEmptyValues)
        .about("comma separated list of categories to set")
        .long("category")
        .short('c');

    let description = Arg::new("description")
        .about("description of the file")
        .long("description")
        .short('d')
        .alias("desc")
        .takes_value(true);

    let media_type = Arg::new("type")
        .long("type")
        .short('T')
        .about("media type of the file")
        .takes_value(true)
        .setting(IgnoreCase)
        .possible_values(MEDIA_TYPES);

    let file = Arg::new("file")
        .multiple(true)
        .required(true)
        .about("the file to set the metadata of");

    let art = Arg::new("artwork")
        .long("artwork")
        .visible_alias("art")
        .about("A jpeg, bmp or a png file.")
        .takes_value(true)
        .setting(AllowEmptyValues);

    let md = ArgGroup::new("md").multiple(true).required(true).args(&[
        "title",
        "artist",
        "album",
        "genre",
        "category",
        "description",
        "type",
        "artwork",
    ]);

    app.arg(artist)
        .arg(album)
        .arg(title)
        .arg(genre)
        .arg(category)
        .arg(media_type)
        .arg(description)
        .arg(file)
        .arg(art)
        .group(md)
}

fn app_get() -> App<'static> {
    App::new("get")
        .visible_alias("g")
        .alias("show")
        .about("Show metadata for a file.")
        .arg(
            Arg::new("file")
                .required(true)
                .multiple(true)
                .about("file to read the metadata of"),
        )
}
