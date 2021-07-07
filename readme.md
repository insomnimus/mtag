# mtag

Mtag is a utility tool that tags mpeg-4  based media files.

# Features

-	Edit artist, album, title, genre etc.
-	Change artwork.
-	Supports any mpeg-4 media type including m4a, m4b, mp4 and more.
-	Very fast, operations take milliseconds.
-	Works with iTunes.

# Supported Formats

Any format that uses mpeg-4 would work, but i tested it with m4b and m4a files and it works.

# Installation

This tool is written in rust, so get a rust toolchain on your system and you're good.

You have two options:

## Install After Git Clone

With this method, you'll also have auto generated shell completions after you build the crate.

```sh
git clone https://github.com/insomnimus/mtag
cd mtag
git checkout main
cargo install --path .
```

## Install From crates.io

`cargo install mtag`
