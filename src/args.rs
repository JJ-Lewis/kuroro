use clap::{
    Args,
    Parser,
    Subcommand
};

/// Program to assist with Kuroro Beast type battling
#[derive(Debug, Parser)]
#[command(name = "Kuroro Matcher")]
#[command(author = "JJLewis")]
#[command(version = "1.0")]
#[command(about = "Your Kuroro Beasts Utility Program", long_about = None)]
pub struct KuroroArgs {
    /// Flag (-e, -b)
    #[clap(subcommand)]
    pub subcommand: Flag
}

#[derive(Debug, Subcommand)]
pub enum Flag {
    /// All Element related information
    Element(ElementCommand),
    Beast(BeastCommand)
}

#[derive(Debug, Args)]
pub struct BeastCommand {
    #[clap(subcommand)]
    pub command: BeastSubCommand
}

#[derive(Debug, Subcommand)]
pub enum BeastSubCommand {
    /// show specific beast details
    Show(ShowBeast),
    /// show specific beast matchups
    Match(MatchBeast),
    /// list all beasts
    List
}

#[derive (Debug, Args)]
pub struct ShowBeast {
    /// name of the Beast
    name: String
}

#[derive (Debug, Args)]
pub struct MatchBeast {
    /// name of the first beast
    first: String,
    /// name of the second beast
    second: String
}

#[derive(Debug, Args)]
pub struct ElementCommand {
    #[clap(subcommand)]
    pub command: ElementSubCommand
}

#[derive(Debug, Subcommand)]
pub enum ElementSubCommand {
    /// Show specific element
    Show(ShowElement),
    /// List all elements
    List
}

#[derive(Debug, Args)]
pub struct ShowElement {
    // name of the element
    pub name: String
}