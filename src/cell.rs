use std::fmt;
use std::iter::FromIterator;

/// Cell holds a character with a style value
#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub chr: char,
    pub style: Option<String>,
    pub end: Option<String>,
}

impl Cell {
    /// Create a new Cell
    pub fn new(chr: char) -> Self {
        Self {
            chr,
            style: None,
            end: None,
        }
    }

    /// Set style of Cell
    pub fn with_style(&mut self, style: &str) -> &mut Self {
        self.style = Some(style.to_string());
        self
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell = match (&self.style, &self.chr, &self.end) {
            (Some(s), c, None) => format!("{}{}", s, c),
            (None, c, Some(e)) => format!("{}{}", c, e),
            (Some(s), c, Some(e)) => format!("{}{}{}", s, c, e),
            (None, c, None) => c.to_string(),
        };
        write!(f, "{}", cell)
    }
}

impl From<(char, Option<String>, Option<String>)> for Cell {
    fn from((chr, style, end): (char, Option<String>, Option<String>)) -> Self {
        Self { chr, style, end }
    }
}

impl From<char> for Cell {
    fn from(chr: char) -> Self {
        Self {
            chr,
            style: None,
            end: None,
        }
    }
}

impl<'a> FromIterator<&'a Cell> for String {
    fn from_iter<I: IntoIterator<Item = &'a Cell>>(iter: I) -> Self {
        let mut string = String::new();
        for c in iter {
            string.push_str(&c.to_string());
        }
        string
    }
}
