#[macro_use]
extern crate clap;
use clap::{App, Arg};

fn main() {
    let user_commands = App::new("Gimme")
        .version(env!("CARGO_PKG_VERSION"))
        .author(crate_authors!())
        .about("Pull useful data out of your clipboard, a file, or a web page.")
        .version(crate_version!())
        .arg(
            Arg::with_name("version")
                .long("version")
                .help("Print the current version of Gimme")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("email")
                .long("email")
                .help("Find all emails or email approximates in the source."),
        )
        .get_matches();

    if user_commands.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }
}
