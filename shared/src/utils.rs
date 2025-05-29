/// Checks whether a username is valid.
pub fn is_username_valid(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
}
