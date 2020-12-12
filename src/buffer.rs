use std::fmt;
use crate::{Cell, Line, IterBuffer, IterMutBuffer, Queued};

/// A Buffer is rectangular data structure that can provide a easy way to arrange
/// text with color in a terminal.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub(crate) cells: Vec<Cell>,
    blank: char,
    queue: Vec<Queued>,
}

// Private
impl Buffer {
    fn create_cells(width: usize, height: usize, blank: char) -> Vec<Cell> {
        // Builds a Vec of Cells with a width and height.
        let mut lines = Vec::new();
        (0..height*width).for_each(
                |_| lines.push(Cell::new(blank))
            );
        lines
    }
}

// Public
impl Buffer {
    /// Create a new buffer with a width and height and background character (aka blank).
    pub fn new(width: usize, height: usize, blank: char) -> Self {
        let cells = Self::create_cells(width,height, blank);
        Self { width, height, cells, blank, queue: Vec::new(), }
    }


    /// Set size of buffer's width and height.
    pub fn set_size(&mut self, width: usize, height: usize) {
        // TODO: This needs to refresh the Vec size.
        self.width = width;
        self.height = height;
    }

    /// Returns a refrance to the slice of Cells
    pub fn as_slice(&self) -> &[Cell] {
        self.cells.as_slice()
    }

    /// Returns a mutable refrance to the slice of Cells
    pub fn as_mut_slice(&mut self) -> &mut [Cell] {
        self.cells.as_mut_slice()
    }

    /// Insert From a string will break up strings with `\n` and place then
    /// and place them as you would expect them to appear.
    pub fn insert_from_str(&mut self, idx: usize, string: &str) {
        let mut x = (idx % self.width) as u16;
        let y = idx / self.width;
        let string_lines = string.split("\n").collect::<Vec<&str>>();
        let mut lines = string_lines.iter().map(|sl| Line::from(*sl)).collect::<Vec<_>>();
        lines.iter_mut().enumerate().for_each(|(i, l)| {
            self.insert_line(x, (y + i) as u16, l.as_mut_slice());
            x = i as u16;
        });
    }

    /// Insert a single `Cell` at a index position.
    pub fn insert_from_idx(&mut self, idx: usize, cell: Cell) {
        let x = (idx % self.width) as u16;
        let y = (idx / self.width) as u16;
        self.queue.push(Queued::from((x, y, cell.clone())));
        [cell].swap_with_slice(&mut self.cells[idx..idx+1]);
    }

    /// Insert a `Cell` at a x and y position.
    pub fn insert_from_cords(&mut self, x: u16, y: u16, cell: Cell) {
        let idx = y as usize * self.width + x as usize;
        [cell].swap_with_slice(&mut self.cells[idx..]);
    }

    /// Insert a slice from line number.
    pub fn insert_line(&mut self, x: u16, y: u16, line: &mut [Cell]) {
        let start = y as usize * self.width + x as usize;
        let end = start + line.len();
        line.swap_with_slice(&mut self.cells[start..end]);
    }

    /// Insert a vertical line(aka a slice) at a x and y posistion.
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

    /// Takes Mutable Closures with args of &str
    /// NOTE: This may change to queue later.
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

    /// Returns a Option<Vec<Queued>>
    pub fn queue(&mut self) -> Option<Vec<Queued>> {
        if self.queue.is_empty() {
            None
        } else {
            let q = self.queue.clone();
            self.queue.clear();
            Some(q)
        }
    }

    /// Returns a slice of Lines
    /// this is going away
    pub fn get_lines(&self) -> Vec<Line> {
        // FIXME: should be a `&[Line]
        self.cells.chunks(self.width).into_iter().map(|l| Line::from(l)).collect::<Vec<Line>>()
    }

    /// Returns a slice of Lines
    pub fn get_mut_lines(&mut self) -> &mut [Line] {
        // NOTE: if get_lines doesnt return a Slice of Lines then this is a pointless method.
        todo!();
    }

    /// Returns a Iterator
    pub fn iter(&'_ self) -> IterBuffer<'_> {
        IterBuffer { inner: self, index: 0 }
    }

    /// Returns a mutable Iterator
    pub fn iter_mut(&'_ mut self) -> IterMutBuffer<'_> {
        IterMutBuffer { inner: self, index: 0 }
    }
}


impl<'a> fmt::Display for Buffer {
    // FIXME: Clean this up.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = self.cells.chunks(self.width);
        let len = lines.len() - 1;
        for (i, line) in lines.enumerate() {
            if i == len {
                write!(f, "{}", line.iter().map(|c| c.to_string()).collect::<String>())?;
            } else {
                write!(f, "{}\n", line.iter().map(|c| c.to_string()).collect::<String>())?;
            }
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a Buffer {
    type Item = &'a Cell;
    type IntoIter = IterBuffer<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_buffer_new() {
        let window = Buffer::new(5, 5, '#');
        assert_eq!(window.to_string(), "#####\n#####\n#####\n#####\n#####");
    }

    #[test]
    fn test_insert_from_str() {
        let mut window = Buffer::new(5, 5, '#');
        window.insert_from_str(3, "Hello World");
        assert_eq!(window.to_string(), "###He\nllo W\norld#\n#####\n#####");
    }

    #[test]
    fn test_insert_line() {
        let mut window = Buffer::new(5, 5, '#');
        let mut line = Line::from("Hey There");
        window.insert_line(0, 0, line.as_mut_slice());
        assert_eq!(window.to_string(), "Hey T\nhere#\n#####\n#####\n#####");
    }

    #[test]
    fn test_insert_vline() {
        let mut window = Buffer::new(5, 5, '#');
        let mut line = Line::from("Hey");
        window.insert_vline(0, 0, line.as_mut_slice());
        assert_eq!(window.to_string(), "H####\ne####\ny####\n#####\n#####");
        let mut line = Line::from("Hey there");
        window.insert_vline(0, 0, line.as_mut_slice());
        assert_eq!(window.to_string(), "H####\ne####\ny####\n ####\nt####");
    }

    #[test]
    fn test_get_line() {
        let window = Buffer::new(5, 5, '#');
        let lines = window.get_lines();
        assert_eq!(lines, vec![Line::from("#####"), Line::from("#####") ,Line::from("#####") ,Line::from("#####") ,Line::from("#####"),]);
    }
}


