use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub target: Target,
}

#[derive(Debug, Subcommand)]
pub enum Target {
    Numpad { abbr: Vec<String> },
    Abbreviate { nump: Vec<String> },
}
