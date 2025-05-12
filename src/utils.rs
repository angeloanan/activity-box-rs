/// Truncate a string to a maximum length, adding `...` to the end if it was
/// truncated.
///
/// ## Arguments
/// * `string` - The string to truncate
/// * `max_length` - The maximum length of the string
pub fn truncate_string(string: impl ToString, max_length: usize) -> String {
    let string = string.to_string();
    if string.len() <= max_length {
        return string;
    }

    format!("{}...", &string[..max_length - 3])
}
