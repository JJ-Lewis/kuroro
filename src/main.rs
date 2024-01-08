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
                args::BeastSubCommand::Show(_c) => {
                    match beasts::show_beast(&_c.name) {
                        None => {println!("No beast found with that name.");return}
                        Some(x) => {
                            // print out pretty format
                            println!("<{}>", x.name.to_uppercase());
                            println!("Elements: {:?}", x.elements);
                        }
                    }
                },
                args::BeastSubCommand::Match(_c) => {
                    match beasts::match_beasts(&_c.first, &_c.second) {
                        None => {println!("One or more beasts not found with those names.")},
                        Some(x) => { println!("{}", x.as_str())}
                    }
                },
                args::BeastSubCommand::Swap(_c) => {
                    println!("SWAP TO: {:#?}", beasts::swap_to(&_c.name));
                },
                args::BeastSubCommand::List => {
                    // just print out all the elements
                    println!("BEASTS:");
                    for value in &*beasts::BEASTS {
                        println!("{:#?}", value)
                    }
                }
            }
         }
    }
}
