//! # Gimme
//! 
//! A library and binary cli for finding things in a bunch of text.  CTRL + A, CTRL + C to pull everything into your clipboard. 
//! Search your clipboard and return any useful data like emails, phone numbers, etc.

#[macro_use]
extern crate lazy_static;

/// Useful data for contacting people.  Emails, phone numbers, etc.
pub mod contacts;
/// Where to search for data, clipboard is only supported source rn
pub mod sources;

/// URL type hyperlinks aka links
pub mod hyperlinks;