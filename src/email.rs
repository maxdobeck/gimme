
pub trait StringExt {
    fn is_email(&self) -> bool;
}

impl StringExt for String {
    fn is_email(&self) -> bool {
        return false
    }
}

impl StringExt for &str {
    fn is_email(&self) -> bool {
        return false
    }
}

#[cfg(test)]
mod tests {
    use super::StringExt;

    #[test]
    fn should_not_be_email() {
        assert_eq!("hello".to_string().is_email(), false);
        assert_eq!("hello again".is_email(), false)
    }
}