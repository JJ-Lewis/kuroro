mod elements;
mod beasts;
mod args;

use args::KuroroArgs;
use clap::Parser;
fn main() {
    // command line arguments -> -e Fire outputs Fire element struct
    // -b Name outputs beast struct
    let args = KuroroArgs::parse();
}
