use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    download: Download,
    subreddits: HashMap<String, Subreddit>,
    aspect_ratio: AspectRatio,
    minimum_size: MinimumSize,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct AspectRatio {
    pub enable: bool,
    pub height: u32,
    pub width: u32,
    pub range: f32,
}

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct MinimumSize {
    pub enable: bool,
    pub height: u32,
    pub width: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Download {
    pub path: PathBuf,
    pub connect_timeout: u32,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Subreddit {
    pub nsfw: bool,
    pub download_first: bool,
    pub sort: Sort,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Hot,
    New,
    Rising,
    Controversial,
    Top,
}
