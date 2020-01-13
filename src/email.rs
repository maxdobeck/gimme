extern crate regex;

use regex::Regex;

pub trait StringExt {
    fn is_email(&self) -> Option<String>;
}

impl StringExt for &str {
    fn is_email(&self) -> Option<String> {
        if strict_email(&self) {
            return Some(self.to_string());
        }
        return None;
    }
}

pub fn find_emails(source: String) -> Vec<String> {
    let emails: Vec<String> = source
        .split_whitespace()
        .filter_map(|word| word.is_email())
        .collect();
    return emails;
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
}
