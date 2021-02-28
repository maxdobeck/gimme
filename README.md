# Gimme
https://crates.io/crates/gimme

Tired of sifting through webpages or long emails for someone's contact info? Pull email/phone numbers out of your clipboard.

1. CTRL + A some large document or web page
1. Run `gimme --email` (or `--phone`) to parse your clipboard for any phone numbers or emails.
1. gimme should spit out anything it finds in the terminal

### Usage
Use `cargo install gimme` or build from source with `cargo build`.

`gimme --help`

Print help message.

`gimme --email`

Find all emails in your clipboard (searches clipboard by default)

`gimme --phone`

Find all potential phone numbers (searches clipboard by default) 

`gimme --link`

Find all hyperlinks (anchor or links to potential URLs)

## Development
### Install + Dev Dependencies
Linux you'll need these to compile:

```xorg-dev pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev```

### Usage
`gimme --help`

Print help message.

`gimme --email`

Find all emails in your clipboard (searches clipboard by default)

` gimme --phone`

Find all potential phone numbers (searches clipboard by default) 

### Crate
https://crates.io/crates/gimme
