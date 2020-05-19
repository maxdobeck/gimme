extern crate regex;
extern crate linkify;

use regex::Regex;
use linkify::{LinkFinder, LinkKind};

pub trait StringExt {
    fn is_email(&self) -> Option<String>;
    fn is_phone(&self) -> Option<String>;
}

impl StringExt for &str {
    fn is_email(&self) -> Option<String> {
        if strict_email(&self) {
            return Some(self.to_string());
        }
        return None;
    }

    fn is_phone(&self) -> Option<String> {
        if valid_phone(&self) {
            return Some(self.to_string());
        }
        return None;
    }
}

pub fn find_emails(source: &str) -> Vec<String> {
    let mut emails: Vec<String> = source
        .split_whitespace()
        .filter_map(|word| word.is_email())
        .collect();
    let mut linkify_emails = double_check_emails(source);
    linkify_emails.sort();
    linkify_emails.dedup();
    emails.append(&mut linkify_emails);
    emails.sort();
    emails.dedup();
    emails
}

fn double_check_emails(source: &str) -> Vec<String> {
    let mut link_finder = LinkFinder::new();
    link_finder.kinds(&[LinkKind::Email]);
    let linkify_emails: Vec<_> = link_finder.links(source).collect();
    let email_str: Vec<&str> = linkify_emails.iter().map(|email| email.as_str()).collect();
    let emails: Vec<String> = email_str.iter().map(|email| email.to_string()).collect();
    emails
}

pub fn find_phone_nums(source: &str) -> Vec<String> {
    let mut phone_nums: Vec<String> = source
        .split_whitespace()
        .filter_map(|word| word.is_phone())
        .collect();
    phone_nums.sort();
    phone_nums.dedup();
    phone_nums
}

fn strict_email(text: &str) -> bool {
    if text.chars().filter(|&c| c == '@').count() > 1 {
        return false;
    }

    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            @
            [[:word:]]+
            \.
            [[:word:]]+$
            "
        )
        .unwrap();
    }
    if RE.is_match(text) {
        return true;
    };
    return false;
}

fn valid_phone(text: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)
            (?:\+?1)?                       # Country Code Optional
            [\s\.]?
            (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
            [\s\.\-]?
            ([2-9]\d{2})                    # Exchange Code
            [\s\.\-]?
            (\d{4})                         # Subscriber Number"
        )
        .unwrap();
    }
    if RE.is_match(text) {
        return true;
    };
    return false;
}

#[cfg(test)]
mod tests {
    use super::StringExt;

    #[test]
    fn should_not_be_email() {
        assert_eq!("hello".is_email(), None);
        assert_eq!("hello again".is_email(), None)
    }

    #[test]
    fn should_be_email() {
        // use https://en.wikipedia.org/wiki/Email_address for valid/invalid email examples
        assert_eq!(super::strict_email("my_email@example.com"), true);
        assert_eq!(super::strict_email("my_emaildomain.com"), false);
        assert_eq!(super::strict_email("my_email@domaincom"), false);
        assert_eq!(super::strict_email("my_emaildomaincom"), false);
        assert_eq!(super::strict_email("my.email+1@example.com"), true);
        assert_eq!(super::strict_email("fname1202@domain.com"), true);
        assert_eq!(super::strict_email("user%example.com@example.org"), true);
        assert_eq!(super::strict_email("@example.com"), true);
        assert_eq!(super::strict_email("wrong@email@example.com"), false);
    }

    #[test]
    fn no_email_duplicates() {
        let sample = "hello my email is frank.roosevelt@whitehouse.gov, one more time that is frank.roosevelt@whitehouse.gov.  Just to be sure... frank.roosevelt@whitehouse.gov";
        let emails = super::find_emails(&sample);
        assert_eq!(emails.len(), 1)
    }

    #[test]
    fn valid_phone_number() {
        vec![
            "18005551234",
            "5553920011",
            "1 (800) 233-2010",
            "+1 916 222-4444",
            "+86 800 555 1234",
        ]
        .iter()
        .for_each(|n| assert_eq!(super::valid_phone(n), true));
    }

    #[test]
    fn invalid_phone_number() {
        vec!["123", "1", "(800)", "+1"]
            .iter()
            .for_each(|n| assert_eq!(super::valid_phone(n), false));
    }
}
