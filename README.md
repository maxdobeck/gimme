# Gimme
Pull the data you want out of non-traditional sources (sources: clipboard content, a URL, a set of files). Current focus is on clipboard support.  Emails first followed by phone numbers.

For example, if you have a ton of text to go through like in a large email thread, marketing page, csv sheet, whatever... and you need a short list of emails, then copy everything to your clipboard, run `gimme`, and take a look at the sorted output!

## Install + Dev Dependencies
Linux you'll need these to compile:

```sudo apt-get install xorg-dev pkg-config libasound2-dev libssl-dev cmake libfreetype6-dev libexpat1-dev libxcb-composite0-dev```

## Usage
`gimme --help`

Print help message.

`gimme --email`

Find all emails in your clipboard (searches clipboard by default)

` gimme --phone`

Find all potential phone numbers (searches clipboard by default) 

### Crate
https://crates.io/crates/gimme
