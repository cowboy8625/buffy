use std::iter::FromIterator;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    chr: char,
}

impl Cell {
    pub fn new(chr: char) -> Self {
        Self { chr }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.chr)
    }
}

impl<'a> FromIterator<&'a Cell> for String {
    fn from_iter<I: IntoIterator<Item=&'a Cell>>(iter: I) -> Self {
        let mut string = String::new();
        for c in iter {
            string.push(c.chr);
        }
        string
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    chars: Vec<Cell>,
}

impl Line {
    pub fn new() -> Self {
        Self { chars: Vec::new() }
    }

    pub fn as_slice(&self) -> &[Cell] {
        self.chars.as_slice()
    }
}

impl From<&str> for Line {
    fn from(string: &str) -> Self {
        let mut chars = Vec::new();
        for c in string.chars() {
            chars.push(Cell::new(c));
        }
        Self { chars }
    }
}

impl<'a> FromIterator<&'a str> for Line {
    fn from_iter<I: IntoIterator<Item=&'a str>>(iter: I) -> Self {
        let string: String = iter.into_iter().collect();
        let chars: Vec<Cell> = string.chars().map(|c| Cell::new(c)).collect();
        Self { chars }
    }
}

#[derive(Debug, Clone)]
pub struct Queue {
    start: usize,     // this is a idx
    end: usize,       // this is a idx
    slice: Vec<Cell>,
}


#[derive(Debug, Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    queue: Vec<Queue>,
}


impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = Self::create_cells(width,height);
        Self { width, height, cells, queue: Vec::new() }
    }

    fn create_cells(width: usize, height: usize) -> Vec<Cell> {
        let mut lines = Vec::new();
        (0..height*width).for_each(
                |_| lines.push(Cell::new('&'))
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

    pub fn insert_vline(&mut self, row_num: u16, line: &[Cell]) {
        assert!((row_num as usize) < self.width);
        let mut line_iter = line.iter();
        for line in self.cells.chunks_mut(self.width) {
            if let Some(cell) = line_iter.next() {
                line[row_num as usize] = *cell;
            }
        }
    }

    pub fn draw<F: FnMut(&str)>(&self, func: &mut F) {
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

impl fmt::Display for Buffer {
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

