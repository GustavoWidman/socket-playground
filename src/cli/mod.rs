mod enums;
pub use enums::*;

use std::{net::Ipv4Addr, path::PathBuf};

use clap::Parser;
use log::LevelFilter;

#[derive(Parser, Debug)]
pub struct Args {
    /// Sets the logger's verbosity level
    #[arg(long, short, default_value_t = LevelFilter::Info)]
    pub verbosity: LevelFilter,

    /// Allows the user to specify the mode of operation
    #[arg(long, short, default_value_t = Mode::Client)]
    pub mode: Mode,

    /// Set the IP address to bind to (or connect to)
    #[arg(long, short)]
    pub addr: Ipv4Addr,

    /// Set the port to bind to (or connect to)
    #[arg(long, short)]
    pub port: u16,

    /// Set the file to read (or write) from (or to)
    #[arg(long, short)]
    pub file: Option<PathBuf>,
}
