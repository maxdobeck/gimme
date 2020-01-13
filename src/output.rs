/// Nicely print the email(s) found
pub fn print_emails(emails: Vec<&str>) {
    for email in emails {
        println!("{}", email);
    }
}
