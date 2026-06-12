fn normalize_line_endings(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter().peekable();
    while let Some(c) = iter.next() {
        match c {
            b'\r' => {
                if iter.peek() == Some(&&b'\n') {
                    iter.next();
                }
                out.push(b'\n');
            }
            _ => out.push(*c),
        }
    }
    out
}

fn replace_trigraphs(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len());
    let mut iter = bytes.iter().peekable();
    while let Some(c) = iter.next() {
        if *c == b'?' && iter.peek() == Some(&&b'?') {
            iter.next();
            match iter.next() {
                Some(b'=') => out.push(b'#'),
                Some(b'(') => out.push(b'['),
                Some(b'/') => out.push(b'\\'),
                Some(b')') => out.push(b']'),
                Some(b'\'') => out.push(b'^'),
                Some(b'<') => out.push(b'{'),
                Some(b'!') => out.push(b'|'),
                Some(b'>') => out.push(b'}'),
                Some(b'-') => out.push(b'~'),
                Some(other) => {
                    out.push(b'?');
                    out.push(b'?');
                    out.push(*other);
                }
                None => {
                    out.push(b'?');
                    out.push(b'?');
                }
            }
        } else {
            out.push(*c);
        }
    }
    out
}

pub fn translation_phase_1(bytes: &[u8]) -> Vec<u8> {
    replace_trigraphs(&normalize_line_endings(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_line_endings() {
        // GIVEN
        let bytes = "A\rB\nC\r\nD\n\rE".as_bytes();
        // WHEN
        let result = normalize_line_endings(bytes);
        // THEN
        let expected = "A\nB\nC\nD\n\nE".as_bytes();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_replace_trigraphs() {
        // GIVEN
        let bytes = b"??= ??( ??/ ??) ??' ??< ??! ??> ??-";
        // WHEN
        let result = replace_trigraphs(bytes);
        // THEN
        let expected = b"# [ \\ ] ^ { | } ~";
        assert_eq!(result, expected);
    }

    #[test]
    fn test_non_trigraphs_are_unchanged() {
        // GIVEN
        let bytes = b"?? ??x ?x= ????";
        // WHEN
        let result = replace_trigraphs(bytes);
        // THEN
        assert_eq!(result, bytes);
    }
}
