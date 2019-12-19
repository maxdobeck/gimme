extern crate regex;

use regex::Regex;

pub trait StringExt {
    fn is_email(&self) -> bool;
}

impl StringExt for String {
    fn is_email(&self) -> bool {
        let s = &self[..];
        return strict_email(s);
    }
}

impl StringExt for &str {
    fn is_email(&self) -> bool {
        return strict_email(&self);
    }
}

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
        assert_eq!("hello".to_string().is_email(), false);
        assert_eq!("hello again".is_email(), false)
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
