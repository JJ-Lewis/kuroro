use clap::{
    Args,
    Parser,
    Subcommand
};

/// Program to assist with Kuroro Beast type battling
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct KuroroArgs {
    /// FIRST ARGUMENT
    pub first_argument: String
}