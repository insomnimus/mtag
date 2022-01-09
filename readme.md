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
## From Scoop (Windows)
First add [my bucket](https://github.com/insomnimus/scoop-bucket) to scoop:

`scoop bucket add insomnia https://github.com/inssomnimus/scoop-bucket`

Update scoop:

`scoop update`

Install the app:

`scoop install mtag`

## Download a Release Binary
From [the releases](https://github.com/insomnimus/mtag/releases) page.

## Build Your Own Binary
With this method, you'll also have auto generated shell completions after you build the crate. 
The generated shell completions will be in the `target/release` directory.

```sh
git clone https://github.com/insomnimus/mtag
cd mtag
git checkout main
cargo install --path .
```

## Install From crates.io
`cargo install mtag`
