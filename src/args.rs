use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Url to the video
    #[arg(short, long)]
    pub url: String,

    /// Quality of the video
    #[arg(value_enum, short, long, default_value_t = Quality::Source)]
    pub quality: Quality,

    /// Output file
    #[arg(short, long)]
    pub output_path: Option<PathBuf>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Debug, ValueEnum)]
pub enum Quality {
    /// Standard Definition (480p)
    SD,
    /// High Definition (720p)
    HD,
    /// Full High Definition (1080p)
    FHD,
    /// Source Quality
    Source,
}
