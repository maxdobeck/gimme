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

/// https://en.wikipedia.org/wiki/Email_address use the invalid email address section for testing
fn strict_email(text: &str) -> bool {
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
        assert_eq!(super::strict_email("my_email@domain.com"), true);
        assert_eq!(super::strict_email("my_emaildomain.com"), false);
        assert_eq!(super::strict_email("my_email@domaincom"), false);
        assert_eq!(super::strict_email("my_emaildomaincom"), false);
        assert_eq!(super::strict_email("my.email+1@domain.com"), true);
        assert_eq!(super::strict_email("fname1202@domain.com"), true);
    }
}
