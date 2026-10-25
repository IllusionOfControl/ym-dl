use clap::Parser;
use serde::Deserialize;
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(name = "Yandex Music Downloader", version = env!("CARGO_PKG_VERSION"))]
pub struct CliArgs {
    #[clap(short, long)]
    pub token: Option<String>,

    #[clap(short, long)]
    pub format: Option<u8>,

    #[clap(short, long)]
    pub get_original_covers: bool,

    #[clap(short, long)]
    pub keep_covers: bool,

    #[clap(short, long)]
    pub out_path: Option<PathBuf>,

    #[clap(short, long)]
    pub sleep: bool,

    #[clap(long)]
    pub write_covers: bool,

    #[clap(long)]
    pub write_lyrics: bool,

    #[clap(long)]
    pub album_template: Option<String>,

    #[clap(long)]
    pub track_template: Option<String>,

    #[clap(short, long, num_args = 1.., required = true)]
    pub urls: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct FileConfig {
    pub token: Option<String>,
    pub ffmpeg_path: Option<PathBuf>,
    pub format: Option<u8>,
    pub out_path: Option<PathBuf>,
    pub keep_covers: Option<bool>,
    pub get_original_covers: Option<bool>,
    pub write_covers: Option<bool>,
    pub write_lyrics: Option<bool>,
    pub sleep: Option<bool>,
    pub album_template: Option<String>,
    pub track_template: Option<String>,
}

