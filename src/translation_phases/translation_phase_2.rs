
pub fn translation_phase_2(bytes: &[u8]) -> Vec<u8> {
    merge_line_endings_with_backslash(bytes)
}

fn merge_line_endings_with_backslash(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter().peekable();
    while let Some(c) = iter.next() {
        match c {
            b'\\' => {
                if iter.peek() == Some(&&b'\n') {
                    iter.next();
                } else {
                    out.push(*c);
                }
            }
            _ => out.push(*c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_line_endings_with_backslash() {
        // GIVEN
        let bytes = "A\\\nB".as_bytes();
        // WHEN
        let result = merge_line_endings_with_backslash(bytes);
        // THEN
        let expected = "AB".as_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_merge_line_endings_with_backslash() {
        // GIVEN
        let bytes = "A\\\\\n\nB".as_bytes();
        // WHEN
        let result = merge_line_endings_with_backslash(bytes);
        // THEN
        let expected = "A\\\nB".as_bytes();
        assert_eq!(result, expected);
    }

}
