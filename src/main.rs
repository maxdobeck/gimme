#[macro_use]
extern crate clap;
use clap::{App, Arg};
use gimme::sources;

fn main() {
    let cmds = App::new("Gimme")
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
                .multiple(false)
                .takes_value(false)
                .help("Find all emails or email approximates."),
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }

    let cb = sources::get_clipboard();

    if cmds.is_present("email") {
        match cb.len() {
            0 => println!("Clipboard is empty!"),
            _ => cb
                .split_whitespace()
                .enumerate()
                .for_each(|(_, word)| println!("{}", word)),
        }
    };
}
