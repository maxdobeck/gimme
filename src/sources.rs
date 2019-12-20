extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

#[derive(Debug)]
pub enum Resource {
    File,
    Clipboard,
    URL,
}

pub fn supported(d: Resource) {
    match d {
        Resource::URL => println!("Parsing from URLs is not supported yet. :/"),
        Resource::Clipboard => println!("Parsing from clipboards is not supported yet. :/"),
        Resource::File => println!("Parsing from a File is not supported yet. :/"),
    };
}

pub fn get_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match ctx.get_contents() {
        Ok(contents) => return contents,
        Err(e) => panic!("{}", e)
    };
}