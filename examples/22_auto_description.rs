#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    App::new("myapp")
       // use crate_description! to pull the description from the Cargo.toml
       .about(crate_description!())
       .get_matches();

    // running the this app with the -h will display whatever app description
    // is in your Cargo.toml
}
