pub fn new() -> App<'static> {
	App::new("abtag")
	.about("Edit mp4 metadata.")
	.setting(AppSettings::SubcommandRequired)
	.global_setting(AppSettings::VersionlessSubcommands)
	.global_setting(AppSettings::UnifiedHelpMessage)
	.subcommand(app_clear())
	.subcommand(app_set())
}

fn app_clear() -> App<'static> {
	App::new("clear")
	.visible_alias("c")
	.aliases(&["clean", "purge"])
	.about("Clear all metadata.")
	.arg(Arg::new("file")
	.multiple(true)
	.about("file to clear the metadata of")
	.required(true),
	)
}

fn app_set() -> App<'static> {
	let app = App::new("set")
	.visible_alias("s")
	.about("Set media metadata.");
	
	let artist = Arg::new("artist")
	.long("artist")
	.visible_alias("art")
	.takes_value(true)
	.setting(UseValueDelimiter)
	.setting(AllowEmptyValues)
	.about("set the artist metadata")
	.group("md")
	.required_unless_present("md");
	
	let title = Arg::new("title")
	.long("title")
	.short_alias('t')
	.alias("tit")
	.visible_alias("ttl")
	.takes_value(true)
	.setting(AllowEmptyValues)
	.about("set the title metadata")
	.group("md")
	.required_unless_present("md");
	
	let album = Arg::new("album")
	.long("album")
	.visible_alias("alb")
	.takes_value(true)
	.setting(AllowEmptyValues)
	.about("set the album metadata")
	.group("md")
	.required_unless_present("md");
	
	let genre = Arg::new("genre")
	.long("genre")
	.visible_alias("gen")
	.short_alias('g')
	.takes_value(true)
	.setting(UseValueDelimiter)
	.setting(AllowEmptyValues)
	.about("set the genre metadata")
	.group("md")
	.required_unless_present("md");
	
	
	let file = Arg::new("file")
	.multiple(true)
	.required(true)
	.about("the file to set the metadata of");
	
	app
	.arg(artist)
	.arg(album)
	.arg(title)
	.arg(genre)
	.arg(file)
}

fn app_get() -> App<'static> {
	App::new("get")
	.visible_alias("g")
	.alias("show")
	.about("Show metadata for a file")
	.arg(Arg::new("file")
	.required(true)
	.multiple(true)
	.about("file to read the metadata of"))
}
