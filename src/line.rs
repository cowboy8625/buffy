use std::iter::FromIterator;
use std::fmt;
use crate::Cell;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    cells: Vec<Cell>,
}

impl Line {
    /// Returns a length of Line as usize.
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    /// Inserts at position.
    pub fn insert(&mut self, idx: usize, cell: Cell) {
        self.cells.insert(idx, cell);
    }

    /// Returns a true if empty.
    pub fn is_empty(&self) -> bool {
        if self.cells.len() == 0 { true } else { false }
    }

    /// Returns a refrance to slice
    pub fn as_slice(&self) -> &[Cell] {
        self.cells.as_slice()
    }

    /// Returns a mutable refrance to slice
    pub fn as_mut_slice(&mut self) -> &mut [Cell] {
        self.cells.as_mut_slice()
    }

    pub fn with_style(mut self, fg: &str) -> Self {
        if let Some(c) = self.cells.get_mut(0) {
            c.style = Some(fg.to_string());
        }
        if let Some(c) = self.cells.last_mut() {
            if let None = c.style {
                c.style = Some("\x1b[0m".to_string());

            }
        }
        self
    }

    pub fn center(&mut self, _width: u16) {
        todo!();
    }
}

impl Default for Line {
    fn default() -> Self {
        Self { cells: Vec::new() }
    }
}

impl From<(&str, &str)> for Line {
    fn from((string, color): (&str, &str)) -> Self {
        let line = Self::from(string).with_style(color);
        line
    }
}

impl From<&str> for Line {
    fn from(string: &str) -> Self {
        let (start, text, end) = strip_code(string);
        let cells: Vec<Cell> = text.chars().map(|c| ( c, start.clone(), end.clone(),).into()).collect();
        Self { cells }
    }
}

impl From<&[Cell]> for Line {
    fn from(cells: &[Cell]) -> Self {
        Self { cells: cells.to_vec() }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let line: String = self.cells.iter().collect();
        write!(f, "{}", line)
    }
}

impl<'a> FromIterator<&'a str> for Line {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        let string: String = iter.into_iter().collect();
        let cells: Vec<_> = string.chars().map(|c| Cell::new(c)).collect();
        Self { cells }
    }
}

fn strip_code(string: &str) ->  (Option<String>, String, Option<String>) {
    let raw = regex::Regex::new("\u{1b}\\[[\\d;]+m").expect("Regex Failed to parse string.").replace_all(string.clone(), "");
    let result: Vec<_> = string.clone().split(&raw.to_string()).collect();
    let start = result.get(0).map(std::string::ToString::to_string).filter(|i| !i.is_empty());
    let end = result.get(1).map(std::string::ToString::to_string).filter(|i| !i.is_empty());
    (start, raw.to_string(), end)
}


