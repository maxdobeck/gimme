#[macro_use]
extern crate clap;
use clap::{App, Arg};
use gimme::sources;

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
                .help("Find all emails or email approximates."),
        )
        .arg(
            Arg::with_name("phone")
                .long("phone")
                .help("Find all phone numbers."),
        )
        .arg(
            Arg::with_name("error")
                .long("error")
                .help("Find any common errors."),
        )
        .arg(
            Arg::with_name("source")
                .long("source")
                .help("The source you want to search in.  Defaults to your clipboard."),
        )
        .get_matches();

    if user_commands.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }
    if user_commands.is_present("email") {}
    if !user_commands.is_present("source") {
        sources::supported(sources::Resource::Clipboard);
    }
    if user_commands.is_present("source") {
        sources::supported(sources::Resource::File);
        sources::supported(sources::Resource::URL);
    }
}
