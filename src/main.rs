mod elements;
mod beasts;
mod args;

use args::{KuroroArgs, Flag, ShowElement};
use clap::Parser;
fn main() {
    // command line arguments -> -e Fire outputs Fire element struct
    // -b Name outputs beast struct
    let args = KuroroArgs::parse();
    // only hits here once satisfying arg structure
    match &args.subcommand {
        Flag::Element (_i) => {
            match &_i.command {
                args::ElementSubCommand::Show (_c) => { println!( "show command - args: {:?}", _c.name)},
                args::ElementSubCommand::List => { println!("list command.")},
            }
         }
    }
}
