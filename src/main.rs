#[macro_use]
extern crate clap;
use clap::{App, Arg};
use gimme::contacts;
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
                .help("Find all emails"),
        )
        .arg(
            Arg::with_name("phone")
                .long("phone")
                .multiple(false)
                .takes_value(false)
                .help("Find all potential phone numbers"),
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }

    let cb = sources::get_clipboard();

    if cmds.is_present("email") {
        // let email_strings = contacts::find_emails(cb);
        let emails =  contacts::find_emails(&cb);
        match emails.len() {
            0 => println!("No emails found"),
            _ => emails.iter().for_each(|e| println!("{}", e)),
        }
    };

    if cmds.is_present("phone") {
        let phone_nums = contacts::find_phone_nums(&cb);
        match phone_nums.len() {
            0 => println!("No phone numbers found"),
            _ => phone_nums.iter().for_each(|e| println!("{}", e)),
        }
    };
}
