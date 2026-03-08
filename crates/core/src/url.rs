use std::fmt::Write as _;

/// Percent-encode a string for use in URL query parameters (RFC 3986 unreserved chars).
pub fn url_encode(s: &str) -> String {
    let mut result = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push('%');
                let _ = write!(result, "{byte:02X}");
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreserved_chars_unchanged() {
        assert_eq!(url_encode("hello-world_2.0~test"), "hello-world_2.0~test");
    }

    #[test]
    fn spaces_and_special_chars_encoded() {
        assert_eq!(url_encode("a b"), "a%20b");
        assert_eq!(url_encode("foo&bar=baz"), "foo%26bar%3Dbaz");
    }

    #[test]
    fn empty_string() {
        assert_eq!(url_encode(""), "");
    }
}
