use std::fmt;
use crate::{Cell, IterBuffer, IterMutBuffer, Queued};

#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub(crate) cells: Vec<Cell>,
    blank: char,
    queue: Vec<Queued>,
}


impl Buffer {
    pub fn new(width: usize, height: usize, blank: char) -> Self {
        let cells = Self::create_cells(width,height, blank);
        Self { width, height, cells, blank, queue: Vec::new() }
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
        let x = (idx % self.width) as u16;
        let y = (idx / self.width) as u16;
        self.queue.push(Queued::from((x, y, cell.clone())));
        [cell].swap_with_slice(&mut self.cells[idx..idx+1]);
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

    pub fn iter(&'_ self) -> IterBuffer<'_> {
        IterBuffer { inner: self, index: 0 }
    }

    pub fn iter_mut(&'_ mut self) -> IterMutBuffer<'_> {
        IterMutBuffer { inner: self, index: 0 }
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

impl<'a> IntoIterator for &'a Buffer {
    type Item = &'a Cell;
    type IntoIter = IterBuffer<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
