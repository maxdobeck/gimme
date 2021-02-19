# Gimme
Tired of sifting through webpages or long emails for someone's contact info? Pull email/phone numbers out of your clipboard. CTRL + A some large document or web page, then run `gimme` to show anything that could be a phone number or email.

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
