#[macro_use]
extern crate clap;
use clap::{App, Arg};
use email::StringExt;
use gimme::email;
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
        // .arg(
        //     Arg::with_name("phone")
        //         .long("phone")
        //         .multiple(false)
        //         .takes_value(false)
        //         .help("Find all phone numbers."),
        // )
        // .arg(
        //     Arg::with_name("error")
        //         .long("error")
        //         .takes_value(false)
        //         .multiple(false)
        //         .help("Find any common errors."),
        // )
        .arg(
            Arg::with_name("source")
                .long("source")
                .short("s")
                .help("The source you want to search in.  Defaults to your clipboard."),
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }

    let cb = sources::get_clipboard();

    if cmds.is_present("email") {
        let emails: Vec<String> = cb
            .split_whitespace()
            .filter_map(|word| word.is_email())
            .collect();
        println!("{:?}", emails);
    };
}
