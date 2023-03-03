/// Truncate a string to a maximum length, adding `...` to the end if it was
/// truncated.
///
/// ## Arguments
/// * `string` - The string to truncate
/// * `max_length` - The maximum length of the string
pub fn truncate_string(string: &str, max_length: usize) -> String {
    if string.len() > max_length {
        format!("{}...", &string[..max_length - 3])
    } else {
        string.to_string()
    }
}
