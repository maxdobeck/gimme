extern crate clipboard;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

/// Get the contents of the clipboard.  Works on all systems but may require 
/// dependencies on x11 in linux environments.
pub fn get_clipboard() -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    match ctx.get_contents() {
        Ok(contents) => return contents,
        Err(e) => panic!("{}", e),
    };
}
