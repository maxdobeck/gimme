extern crate clap;
use clap::{App, Arg};

fn main() {
    let user_commands = App::new("MyApp")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Max Dobeck. <max.dobeck@protonmail.com>")
        .about("Pull useful data out of some source.")
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Print the current version of Gimme")
                .takes_value(false),
        )
        .get_matches();

        if user_commands.is_present("version") {
            println!("gimme version {}", env!("CARGO_PKG_VERSION"))
        }
}