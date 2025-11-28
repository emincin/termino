pub fn is_all_digits(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    return s.chars().all(|c: char| c.is_ascii_digit());
}
