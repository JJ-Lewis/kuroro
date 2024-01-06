use clap::{
    Args,
    Parser,
    Subcommand
};

/// Program to assist with Kuroro Beast type battling
#[derive(Debug, Parser)]
#[command(name = "MyApp")]
#[command(author = "Kevin K. <kbknapp@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Does awesome things", long_about = None)]
pub struct KuroroArgs {
    /// Flag (-e, -b)
    #[clap(subcommand)]
    pub subcommand: Flag
}

#[derive(Debug, Subcommand)]
pub enum Flag {
    /// All Element related information
    Element(ElementCommand),
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