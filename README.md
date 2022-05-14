# ridit-rs
Reddit Image Download written in Rust. Download images that is submitted to Redit with operation scoped upon by subreddits.

# Features

1. Aspect Ratio and size aware. Will not download images that does not meet aspect ratio (how square or not square the iamge is) specification, and filter the minimum size. So low quality pictures and images that does not fit your device target will be filtered.
2. Supports for "Profile". The software can download images for multiple device targets in one go. Images that fit the profile will be downloaded to that profile. Two profiles will be created on first program run, "mobile" and "main". "main" for desktop wallpaper, "mobile" for target devices. 

NOTE: Current version of the application will not download albums.

# Installation

Clone this repo and use cargo to build this.

```sh
cargo install --path .
```

# Usage

## Start Downloading

```sh
ridit start
```

## Add Subreddit

```sh
ridit subreddit add "wallpaper" "wallpapers"
```

## Subreddit Management

```sh
ridit subreddit --help
```

## Profile Management

```sh
ridit profile --help
```

## Print Configuration

```sh
ridit print
```

## Download Management

```sh
ridit download --help
```

# Distribution / Compiling Note

Depending on how you compile this program, this may or may not require depndencies on user machines. To ensure dependency free executable (static linked binaries), please use musl builder.

```sh
$ rustup target add x86_64-unknown-linux-musl
$ cargo build --release --target=x86_64-unknown-linux-musl
```
