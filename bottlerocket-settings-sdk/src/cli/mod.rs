use std::path::PathBuf;

pub mod proto1;

pub use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub protocol: Protocol,
}

impl Cli {
    /// Parse extension arguments from stdin
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}

#[derive(Subcommand, Debug)]
pub enum Protocol {
    /// Settings extension protocol 1
    Proto1(proto1::Protocol1),
}
