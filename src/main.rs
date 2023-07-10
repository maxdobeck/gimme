use clap::{app_from_crate, Arg};
use gimme::contacts;
use gimme::hyperlinks;
use gimme::sources;

fn main() {
    include_str!("../Cargo.toml");
    let cmds = app_from_crate!()
        .arg(
            Arg::new("version")
                .long("version")
                .help("Print the current version of Gimme")
                .takes_value(false),
        )
        .arg(
            Arg::new("email")
                .long("email")
                .takes_value(false)
                .multiple_values(false)
                .help("Find all emails"),
        )
        .arg(
            Arg::new("phone")
                .long("phone")
                .takes_value(false)
                .help("Find all potential phone numbers")
                .multiple_values(false),
        )
        .arg(
            Arg::new("link")
                .long("link")
                .multiple_values(false)
                .takes_value(false)
                .help("Find all URL hyperlinks"),
        )
        .get_matches();

    if cmds.is_present("version") {
        println!("gimme version {}", env!("CARGO_PKG_VERSION"))
    }

    let cb = sources::get_clipboard();

    if cmds.is_present("email") {
        let emails = contacts::find_emails(&cb);
        match emails.len() {
            0 => println!("No emails found"),
            _ => emails.iter().for_each(|e| println!("{}", e)),
        }
    };

    if cmds.is_present("phone") {
        let phone_nums = contacts::find_phone_nums(&cb);
        match phone_nums.len() {
            0 => println!("No phone numbers found"),
            _ => phone_nums.iter().for_each(|p| println!("{}", p)),
        }
    };

    if cmds.is_present("link") {
        let links = hyperlinks::find_links(&cb);
        match links.len() {
            0 => println!("No links found"),
            _ => links.iter().for_each(|l| println!("{}", l)),
        }
    }
}
