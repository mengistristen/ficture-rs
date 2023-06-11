//! This module contains the structure containing all
//! command line arguments.
pub use clap::Parser;

/// A structure containing all command line arguments.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The width of generated maps.
    #[arg(long, default_value_t = 1920)]
    pub width: usize,

    /// The height of generated maps.
    #[arg(long, default_value_t = 1080)]
    pub height: usize,

    /// The path to the config file to use.
    #[arg(long, short, default_value_t = String::from("config/config.yaml"))]
    pub filepath: String
}
