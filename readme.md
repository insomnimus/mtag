# mtag

Mtag is a utility tool that tags mpeg-4  based media files.

# Supported Formats

Any format that uses mpeg-4 would work, but i tested it with m4b and m4a files and it works.

# Installation

This tool is written in rust, so get a rust toolchain on your system and you're good.

mtag I haven't released mtag to crates.io yet, so for now you have two options:

## Install After Git Clone

```sh
git clone https://github.com/insomnimus/mtag
cd mtag
git checkout main
cargo install --path .
```

## Install Directly With Cargo

`cargo install --git https://github.com/insomnimus/mtag --branch main`
