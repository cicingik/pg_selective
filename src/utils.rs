use std::io::{Error, ErrorKind, Write};
use std::str;

// #[cfg(any(not(windows), not(feature = "win_crlf")))]
// pub static NEWLINE: &[u8] = b"\n";
// #[cfg(all(windows, feature = "win_crlf"))]
// pub static NEWLINE: &[u8] = b"\r\n";

/// Internal utility for writing data into a string
pub struct StringWriter {
    string: String,
}

#[allow(dead_code)]
impl StringWriter {
    /// Create a new `StringWriter`
    pub fn new() -> StringWriter {
        StringWriter {
            string: String::new(),
        }
    }

    /// Return a reference to the internally written `String`
    #[allow(dead_code)]
    pub fn as_string(&self) -> &str {
        &self.string
    }
}

impl Write for StringWriter {
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let string = match str::from_utf8(data) {
            Ok(s) => s,
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Cannot decode utf8 string : {}", e),
                ))
            }
        };
        self.string.push_str(string);
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        // Nothing to do here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn string_writer() {
        let mut out = StringWriter::new();
        out.write_all(b"foo").unwrap();
        out.write_all(b" ").unwrap();
        out.write_all(b"").unwrap();
        out.write_all(b"bar").unwrap();
        assert_eq!(out.as_string(), "foo bar");
    }

    #[test]
    fn utf8_error() {
        let mut out = StringWriter::new();
        let res = out.write_all(&[0, 255]);
        assert!(res.is_err());
    }
}
