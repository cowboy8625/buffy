use std::iter::FromIterator;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Cell<T: fmt::Display + Clone> {
    chr: char,
    fg: Option<T>,
    bg: Option<T>,
}

impl<T: fmt::Display + Clone> Cell<T> {
    pub fn new(chr: char) -> Self {
        Self { chr, fg: None, bg: None }
    }
}

impl<T: fmt::Display + Clone> fmt::Display for Cell<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cell = match (&self.chr, &self.fg, &self.bg) {
            (c, Some(f), None) => format!("{}{}", f, c),
            (c, None, Some(b)) => format!("{}{}", b, c),
            (c, Some(f), Some(b)) => format!("{}{}{}", f, b, c),
            (c, None, None) => c.to_string(),
        };
        write!(f, "{}", cell)
    }
}

impl<'a, T: fmt::Display + Clone + 'a> FromIterator<&'a Cell<T>> for String {
    fn from_iter<I: IntoIterator<Item=&'a Cell<T>>>(iter: I) -> Self {
        let mut string = String::new();
        for c in iter {
            string.push_str(&c.to_string());
        }
        string
    }
}

#[derive(Debug, Clone)]
pub struct Line<T: fmt::Display + Clone> {
    chars: Vec<Cell<T>>,
}

impl<T: fmt::Display + Clone> Line<T> {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }

    pub fn as_slice(&self) -> &[Cell<T>] {
        self.chars.as_slice()
    }
}

impl<T: fmt::Display + Clone> From<&str> for Line<T> {
    fn from(string: &str) -> Self {
        let mut chars = Vec::new();
        for c in string.chars() {
            chars.push(Cell::new(c));
        }
        Self { chars }
    }
}

impl<'a, T: fmt::Display + Clone> FromIterator<&'a str> for Line<T> {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        let string: String = iter.into_iter().collect();
        let chars: Vec<_> = string.chars().map(|c| Cell::new(c)).collect();
        Self { chars }
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
pub struct Buffer<T: fmt::Display + Clone> {
    width: usize,
    height: usize,
    cells: Vec<Cell<T>>,
    //queue: Vec<Queue<T>>,
}


impl<T: fmt::Display + Clone> Buffer<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = Self::create_cells(width,height);
        Self { width, height, cells }
    }

    fn create_cells(width: usize, height: usize) -> Vec<Cell<T>> {
        let mut lines = Vec::new();
        (0..height*width).for_each(
                |_| lines.push(Cell::new('&'))
            );
        lines
    }

    pub fn as_slice(&self) -> &[Cell<T>] {
        self.cells.as_slice()
    }

    pub fn insert_from_idx(&mut self, idx: usize, cell: Cell<T>) {
        [cell].swap_with_slice(&mut self.cells[idx..]);
    }

    pub fn insert_from_cords(&mut self, x: u16, y: u16, cell: Cell<T>) {
        let idx = y as usize * self.width + x as usize;
        [cell].swap_with_slice(&mut self.cells[idx..]);
    }

    pub fn insert_line(&mut self, line_num: u16, line: &mut [Cell<T>]) {
        let start = line_num as usize * self.width;
        let end = start + line.len();
        line.swap_with_slice(&mut self.cells[start..end]);
    }

    pub fn insert_vline(&mut self, row_num: u16, line: &[Cell<T>]) {
        assert!((row_num as usize) < self.width);
        let mut line_iter = line.iter();
        for line in self.cells.chunks_mut(self.width) {
            if let Some(cell) = line_iter.next() {
                line[row_num as usize] = cell.clone();
            }
        }
    }

    pub fn get<F: FnMut(&str)>(&self, func: &mut F) {
        let mut string = String::new();
        for (idx, c) in self.cells.iter().enumerate() {
            if idx != 0 && idx % self.width == 0 {
                string.push('\n');
            }
            string.push(c.chr);
        }
        func(&string);
    }
}

impl<T: fmt::Display + Clone> fmt::Display for Buffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.cells.iter().enumerate().map(|(i, c)|{
            if i != 0 && i % self.width == 0 {
                format!("{}\n", c)
            } else {
                c.chr.to_string()
            }
        }).collect();
        write!(f, "{}", string)
    }
}

