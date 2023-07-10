use clap::command;
use clap::Arg;
use gimme::contacts;
use gimme::hyperlinks;
use gimme::sources;

fn main() {
    let cmds = command!()
        .arg(
            Arg::new("email")
                .long("email")
                .action(clap::ArgAction::SetTrue)
                .help("Find all emails"),
        )
        .arg(
            Arg::new("phone")
                .long("phone")
                .action(clap::ArgAction::SetTrue)
                .help("Find all potential phone numbers"),
        )
        .arg(
            Arg::new("link")
                .long("link")
                .action(clap::ArgAction::SetTrue)
                .help("Find all URL hyperlinks"),
        )
        .get_matches();

    let cb: String = if atty::is(atty::Stream::Stdin) {
        sources::get_clipboard()
    } else {
        sources::get_stdin()
    };

    if cmds.get_flag("email") {
        let emails = contacts::find_emails(&cb);
        match emails.len() {
            0 => println!("No emails found"),
            _ => emails.iter().for_each(|e| println!("{}", e)),
        }
    };

    if cmds.get_flag("phone") {
        let phone_nums = contacts::find_phone_nums(&cb);
        match phone_nums.len() {
            0 => println!("No phone numbers found"),
            _ => phone_nums.iter().for_each(|p| println!("{}", p)),
        }
    };

    if cmds.get_flag("link") {
        let links = hyperlinks::find_links(&cb);
        match links.len() {
            0 => println!("No links found"),
            _ => links.iter().for_each(|l| println!("{}", l)),
        }
    }
}
