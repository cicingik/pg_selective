pub trait SqlString {
    fn sql_identifier(&self) -> Self;
    fn sql_value(&self) -> Self;
}

impl SqlString for String {
    fn sql_value(&self) -> Self {
        format!(r#"'{}'"#, self)
    }

    fn sql_identifier(&self) -> Self {
        if needs_quoting(self) {
            return format!(r#""{}""#, self);
        }
        self.to_owned()
    }
}

fn needs_quoting(string: &str) -> bool {
    let lower = string.to_ascii_lowercase();

    // If it doesn't match the lower case equivalent, assume the column name is actually mixed cased and that we need to quote it.
    if string != lower {
        return true;
    }

    false
}
