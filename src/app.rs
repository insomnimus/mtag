use clap::{
	arg,
	crate_version,
	App,
	AppSettings,
	Arg,
	ArgGroup,
	ValueHint,
};

pub fn new() -> App<'static> {
	App::new("mtag")
		.version(crate_version!())
		.about("Edit mpeg-4 metadata.")
		.global_setting(AppSettings::AllArgsOverrideSelf)
		.global_setting(AppSettings::InferLongArgs)
		.setting(AppSettings::InferSubcommands)
		.setting(AppSettings::UseLongFormatForHelpSubcommand)
		.setting(AppSettings::SubcommandRequiredElseHelp)
		.subcommands([app_clear(), app_get(), app_set()])
}

fn app_get() -> App<'static> {
	App::new("get").about("Display metadata of a file.").arg(
		Arg::new("file")
			.help("One or more media files.")
			.required(true)
			.value_name("FILE")
			.multiple_values(true)
			.value_hint(ValueHint::FilePath),
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
		"",
	];

	macro_rules! args {
			($($x:expr),* $(,)?) => {
				&[
				$($x.default_missing_value("").group("keys")),*
				]
			};
		}
	macro_rules! args_many {
			($($x:expr),* $(,)?) => {
				&[
				$($x
				.default_missing_values(&[])
				.use_delimiter(true)
				.require_delimiter(true)
				.multiple_values(true)
				.group("keys")),*
				]
			};
		}

	App::new("set")
		.about("Set mpeg-4 metadata.")
		.after_help("to clear a specific key, leave out the value or set it to an empty string.")
		.group(ArgGroup::new("keys").multiple(true).required(true))
		.args(args_many![
			Arg::new("artist")
				.help("Comma separated list of artists.")
				.long("artist")
				.takes_value(true),
			Arg::new("genre")
				.help("Comma separated list of genres.")
				.takes_value(true)
				.long("genre"),
			Arg::new("category")
				.help("Comma separated list of categories.")
				.takes_value(true),
		])
		.args(args![
	arg!(--title [TITLE] "The title."),
	arg!(--album [ALBUM] "The album name."),
	arg!(--description [DESCRIPTION] "The description."),
	arg!(--type [TYPE] "The media type.")
	.possible_values(MEDIA_TYPES),
	arg!(--artwork [PATH] "Path to an image file (jpeg, bmp or png) to be used as the artwork.")
	.visible_alias("cover")
	.value_hint(ValueHint::FilePath),
	arg!(--bpm [BPM] "Beats per minute.").validator(validate_u16),
	arg!(--track [TRACK] "The track number in the form N/TOTAL.").validator(validate_track),
	arg!(--disc [DISC] "The disc number in the form N/TOTAL.").validator(validate_track),
	arg!(--copyright [COPYRIGHT] "The copyright information."),
	arg!(--isrc "The ISRC code."),
	arg!(--show [NAME] "The TV show name."),
	arg!(--work [WORK] "The name of the work."),
	arg!(--year [YEAR] "The year."),
	])
		.arg(
			Arg::new("file")
				.help("Path to a media file.")
				.required(true)
				.value_name("FILE")
				.multiple_values(true)
				.value_hint(ValueHint::FilePath),
		)
}

fn app_clear() -> App<'static> {
	App::new("clear").about("Clear metadata from files.").arg(
		Arg::new("file")
			.help("One or more media files.")
			.required(true)
			.multiple_values(true)
			.value_name("FILE")
			.value_hint(ValueHint::FilePath),
	)
}

fn validate_u16(s: &str) -> Result<(), String> {
	if s.is_empty() {
		return Ok(());
	}
	s.parse::<u16>()
		.map(|_| {})
		.map_err(|_| String::from("the value must be a positive integer or 0"))
}

fn validate_track(s: &str) -> Result<(), String> {
	if s.is_empty() {
		return Ok(());
	}
	let (left, right) = s
		.split_once('/')
		.ok_or_else(|| String::from("the correct syntax is N/TOTAL"))?;
	let left = left
		.parse::<u16>()
		.map_err(|_| String::from("the correct syntax is N/TOTAL"))?;
	let right = right
		.parse::<u16>()
		.map_err(|_| String::from("the correct syntax is N/TOTAL"))?;
	if left > right {
		Err(String::from("N/TOTAL: N can't be larger than TOTAL"))
	} else {
		Ok(())
	}
}
