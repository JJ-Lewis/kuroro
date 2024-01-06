mod elements;
mod beasts;
mod args;

use args::{KuroroArgs, Flag};
use clap::Parser;

fn main() {
    // command line arguments -> -e Fire outputs Fire element struct
    // -b Name outputs beast struct
    let args = KuroroArgs::parse();
    // only hits here once satisfying arg structure
    match &args.subcommand {
        // handle all element subcommands
        Flag::Element (_i) => {
            match &_i.command {
                args::ElementSubCommand::Show (_c) => {
                    match elements::show_element(&_c.name) {
                        None => {println!("No element found with that name.");return}
                        Some(x) => {
                            // print out pretty format
                            println!("<{}>", x.name.to_uppercase());
                            println!("Good against: {:?}", x.good_against.join(", "));
                            println!("Bad against: {:?}", x.bad_against.join(", "));
                        }
                    }
                },
                args::ElementSubCommand::List => {
                    // just print out all the elements
                    println!("ELEMENTS:");
                    for value in &*elements::ELEMENTS {
                        println!("{:#?}", value)
                    }
                   
                },
            }
         },
         // handle all beast subcommands
         Flag::Beast (_i) => {
            match &_i.command {
                args::BeastSubCommand::Show(_c) => {},
                args::BeastSubCommand::Match(_c) => {},
                &args::BeastSubCommand::List => {}
            }
         }
    }
}
