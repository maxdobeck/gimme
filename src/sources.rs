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
