extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

pub fn get_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match ctx.get_contents() {
        Ok(contents) => return contents,
        Err(e) => panic!("{}", e),
    };
}
