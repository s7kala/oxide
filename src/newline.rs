/// Expands newlines in the string by calling the output function for each byte,
/// converting '\n' to '\r\n'
pub fn expand_newlines<F>(s: &str, mut output: F)
where
    F: FnMut(u8),
{
    for b in s.bytes() {
        if b == b'\n' {
            output(b'\r');
        }
        output(b);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newline_expansion() {
        let mut result = Vec::new();
        expand_newlines("hello\nworld\n", |b| result.push(b));
        assert_eq!(result, b"hello\r\nworld\r\n");
    }

    #[test]
    fn test_no_newlines() {
        let mut result = Vec::new();
        expand_newlines("hello world", |b| result.push(b));
        assert_eq!(result, b"hello world");
    }

    #[test]
    fn test_multiple_newlines() {
        let mut result = Vec::new();
        expand_newlines("\n\n", |b| result.push(b));
        assert_eq!(result, b"\r\n\r\n");
    }

    #[test]
    fn test_empty_string() {
        let mut result = Vec::new();
        expand_newlines("", |b| result.push(b));
        assert_eq!(result, b"");
    }

    #[test]
    fn test_trailing_non_newline() {
        let mut result = Vec::new();
        expand_newlines("hello\nworld", |b| result.push(b));
        assert_eq!(result, b"hello\r\nworld");
    }
}