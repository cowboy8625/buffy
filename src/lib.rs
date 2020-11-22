use std::iter::FromIterator;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Cell {
    chr: char,
    style: Option<String>,
    end: Option<String>,
}

impl Cell {
    pub fn new(chr: char) -> Self {
        Self { chr, style: None, end: None }
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

impl<'a> FromIterator<&'a Cell> for String {
    fn from_iter<I: IntoIterator<Item=&'a Cell>>(iter: I) -> Self {
        let mut string = String::new();
        for c in iter {
            string.push_str(&c.to_string());
        }
        string
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    cells: Vec<Cell>,
}

impl Line {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn as_slice(&self) -> &[Cell] {
        self.cells.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut [Cell] {
        self.cells.as_mut_slice()
    }

    pub fn style(&mut self, fg: &str) -> &mut Self {
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
}

impl From<&str> for Line {
    fn from(string: &str) -> Self {
        let (start, text, end) = strip_code(string);
        let cells: Vec<Cell> = text.chars().map(|c| ( c, start.clone(), end.clone(),).into()).collect();
        Self { cells }
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

// #[derive(Debug, Clone)]
// pub struct Queue<T: fmt::Display + Clone> {
//     start: usize,     // this is a idx
//     end: usize,       // this is a idx
//     slice: Vec<Cell<T>>,
// }
//

#[derive(Debug, Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    blank: char,
    //queue: Vec<Queue<T>>,
}


impl Buffer {
    pub fn new(width: usize, height: usize, blank: char) -> Self {
        let cells = Self::create_cells(width,height, blank);
        Self { width, height, cells, blank }
    }

    fn create_cells(width: usize, height: usize, blank: char) -> Vec<Cell> {
        let mut lines = Vec::new();
        (0..height*width).for_each(
                |_| lines.push(Cell::new(blank))
            );
        lines
    }

    pub fn as_slice(&self) -> &[Cell] {
        self.cells.as_slice()
    }

    pub fn insert_from_idx(&mut self, idx: usize, cell: Cell) {
        [cell].swap_with_slice(&mut self.cells[idx..]);
    }

    pub fn insert_from_cords(&mut self, x: u16, y: u16, cell: Cell) {
        let idx = y as usize * self.width + x as usize;
        [cell].swap_with_slice(&mut self.cells[idx..]);
    }

    pub fn insert_line(&mut self, line_num: u16, line: &mut [Cell]) {
        let start = line_num as usize * self.width;
        let end = start + line.len();
        line.swap_with_slice(&mut self.cells[start..end]);
    }

    pub fn insert_vline(&mut self, x: u16, y: u16, line: &[Cell]) {
        assert!((x as usize) < self.width);
        assert!((y as usize) < self.height);
        let mut line_iter = line.iter();
        for (yl, line) in self.cells.chunks_mut(self.width).enumerate() {
            if yl >= y as usize {
                if let Some(cell) = line_iter.next() {
                    line[x as usize] = cell.clone();
                }
            }
        }
    }

    pub fn get<F: FnMut(&str)>(&self, func: &mut F) {
        let mut string = String::new();
        for (idx, c) in self.cells.iter().enumerate() {
            if idx != 0 && idx % self.width == 0 {
                string.push('\n');
            }
            string.push_str(&c.to_string());
        }
        func(&string);
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.cells.iter().enumerate().map(|(i, c)|{
            if i != 0 && i % self.width == 0 {
                format!("{}\n", c)
            } else {
                c.to_string()
            }
        }).collect();
        write!(f, "{}", string)
    }
}

fn strip_code(string: &str) ->  (Option<String>, String, Option<String>) {
    let raw = regex::Regex::new("\u{1b}\\[[\\d;]+m").expect("Regex Failed to parse string.").replace_all(string.clone(), "");
    let result: Vec<_> = string.clone().split(&raw.to_string()).collect();
    let start = result.get(0).map(std::string::ToString::to_string).filter(|i| !i.is_empty());
    let end = result.get(1).map(std::string::ToString::to_string).filter(|i| !i.is_empty());
    (start, raw.to_string(), end)
}

