use crate::error::CompilerError;

fn normalize_line_endings(bytes: &[u8]) -> Result<Vec<u8>, CompilerError> {
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
    Ok(out)
}

fn replace_trigraphs(bytes: &[u8]) -> Result<Vec<u8>, CompilerError> {
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
    Ok(out)
}

pub fn translation_phase_1(bytes: &[u8]) -> Result<Vec<u8>, CompilerError> {
    replace_trigraphs(&normalize_line_endings(bytes)?)
}
