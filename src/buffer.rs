use std::fmt;
use crate::{Cell, Line, IterBuffer, IterMutBuffer, Queued, Location};

/// A Buffer is rectangular data structure that can provide a easy way to arrange
/// text with color in a terminal.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub(crate) cells: Vec<Cell>,
    blank: char,
    queue: Vec<Queued>,
    _wrap: bool,// Needs to be a enum of letter/word wrapper types
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
        Self { width, height, cells, blank, queue: Vec::new(), _wrap: false }
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
    pub fn insert_str(&mut self, idx: usize, string: &str) {
        let mut x = (idx % self.width) as u16;
        let y = idx / self.width;
        let string_lines = string.split("\n").collect::<Vec<&str>>();
        let lines = string_lines.iter().map(|sl| Line::from(*sl)).collect::<Vec<_>>();
        lines.into_iter().enumerate().for_each(|(i, l)| {
            dbg!(&x, y + i);
            self.insert_line(&(x, (y + i) as u16).into(), l);
            x = i as u16;
        });
    }

    fn insert_from_idx(&mut self, idx: usize, cell: Cell) {
        // Insert a single `Cell` at a index position.
        let x = (idx % self.width) as u16;
        let y = (idx / self.width) as u16;
        self.queue.push(Queued::from((x, y, cell.clone())));
        [cell].swap_with_slice(&mut self.cells[idx..idx+1]);
    }

    fn insert_from_cords(&mut self, x: u16, y: u16, cell: Cell) {
        // Insert a `Cell` at a x and y position.
        let idx = y as usize * self.width + x as usize;
        self.insert_from_idx(idx, cell);
    }

    pub fn insert_cell(&mut self, loc: &Location, cell: Cell) {
        match loc {
            Location::Index(idx) => self.insert_from_idx(*idx, cell),
            Location::Cords{x, y} => self.insert_from_cords(*x, *y, cell),
        }
    }

    /// Insert a Line from Location into buffer.
    pub fn insert_line(&mut self, loc: &Location, mut line: Line) {
        let (x, y) = match loc {
            Location::Index(idx) => {
                ((idx % self.width) as u16, (idx / self.height) as u16)
            },
            Location::Cords{x, y} => (*x, *y),
        };
        let start = y as usize * self.width + x as usize;
        let total = std::cmp::min((start + line.len()) - start - x as usize, self.width);
        let end = start + total;
        line.resize(total);
        self.queue.push(Queued::from((x, y, &line)));
        line.as_mut_slice().swap_with_slice(&mut self.cells[start..end]);
    }

    /// Insert a vertical line(aka a slice) at a x and y posistion.
    pub fn insert_vline(&mut self, loc: &Location, line: &Line) {
        // FIXME: Added Insert Queued.
        let (x, y) = match loc {
            Location::Index(idx) => {
                ((idx % self.width) as u16, (idx / self.height) as u16)
            },
            Location::Cords{x, y} => (*x, *y),
        };
        assert!((x as usize) < self.width);
        assert!((y as usize) < self.height);
        for (idx, cell) in line.iter().enumerate() {
            if y as usize + idx == self.height { break; }
            self.insert_cell(&Location::Cords{x,y: y + idx as u16}, cell.clone());
        }
        // let mut line_iter = line.iter();
        // for (yl, line) in self.cells.chunks_mut(self.width).enumerate() {
        //     if yl >= y as usize {
        //         if let Some(cell) = line_iter.next() {
        //             self.insert_cell(&Location::Cords{x,y: yl as u16}, cell.clone());
        //             // line[x as usize] = cell.clone();
        //         }
        //     }
        // }
    }

    /// Takes Mutable Closures with args of &str
    /// NOTE: This may change to queue later.
    pub fn get<F: FnMut(&str)>(&self, mut func: F) {
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
    fn test_insert_str() {
        let (width, height) = (5, 5);
        let mut window = Buffer::new(width, height, '#');
        let idx = 3;
        let (x, y) = (idx % width, idx / width);
        window.insert_str(3, "Hello01\nWorld01");
        assert_eq!(window.to_string(), "###He\nWorld\n#####\n#####\n#####");
        let left = window.queue();
        let right = Some(vec![
            Queued::from((x as u16, y as u16, Line::from("Hell"))),
            Queued::from((0, 1, Line::from("World"))),
        ]);
        if let Some(l) = &left {
            for (idx, le) in l.iter().enumerate() {
                let (x, y) = le.cords();
                println!("left: {}: x{}, y{}, {}", idx, x, y, le.to_string());
            }
        }
        if let Some(r) = &right {
            for (idx, le) in r.iter().enumerate() {
                let (x, y) = le.cords();
                println!("right: {}: x{}, y{}, {}", idx, x, y, le.to_string());
            }
        }
        assert_eq!(left, right);
    }

    #[test]
    fn test_insert_from_idx() {
        let (width, height) = (5, 5);
        let mut window = Buffer::new(width, height, '#');
        let idx = 3;
        window.insert_from_idx(idx, Cell::new('c'));
        let (x, y) = (idx % width, idx / width);
        assert_eq!(window.to_string(), "###c#\n#####\n#####\n#####\n#####");
        assert_eq!(window.queue(), Some(vec![Queued::from((x as u16, y as u16, Cell::new('c')))]));
    }

    /// Checks to make sure that the insert_from_cords method not only pushes
    /// a line to the right location in the buffer but also added the line or lines to the queue
    #[test]
    fn test_insert_from_cords() {
        let (width, height) = (5, 5);
        let (x, y) = (1, 1);
        let mut window = Buffer::new(width, height, '#');
        window.insert_from_cords(1, 1, Cell::new('c'));
        assert_eq!(window.to_string(), "#####\n#c###\n#####\n#####\n#####");
        assert_eq!(window.queue(), Some(vec![Queued::from((x, y, Cell::new('c')))]));
    }

    #[test]
    fn test_insert_cell() {
        let (width, height) = (5, 5);
        let (x, y) = (2, 2);
        let idx = 6;
        let mut window = Buffer::new(width, height, '#');
        window.insert_cell(&Location::Index(idx), Cell::new('c'));
        window.insert_cell(&Location::Cords{x,y}, Cell::new('h'));
        assert_eq!(window.to_string(), "#####\n#c###\n##h##\n#####\n#####");
        assert_eq!(window.queue(), Some(vec![
                        Queued::from(((idx%width) as u16, (idx/width) as u16, Cell::new('c'))),
                        Queued::from((x, y, Cell::new('h'))),
        ]));
    }

    /// Checks to make sure that the insert_line method not only pushes
    /// a line to the right location in the buffer but also added the line or lines to the queue
    #[test]
    fn test_insert_line() {
        let mut window = Buffer::new(5, 5, '#');
        let line = Line::from("Hey There");
        let (x, y) = (0, 0);
        let output = "Hey T\n#####\n#####\n#####\n#####";
        window.insert_line(&(x, y).into(), line);
        assert_eq!(window.to_string(), output);
        let output_queue = vec![
            Queued::from((x, y, Line::from("Hey T"))),
        ];
        assert_eq!(window.queue(), Some(output_queue));
    }

    #[test]
    fn test_insert_vline() {
        // FIXME: This should not wrap unless told to.
        let mut window = Buffer::new(5, 5, '#');
        let line = Line::from("Hey");
        window.insert_vline(&(0u16, 0u16).into(), &line);
        assert_eq!(window.to_string(), "H####\ne####\ny####\n#####\n#####");
        let line = Line::from("Hey there");
        window.insert_vline(&(0u16, 0u16).into(), &line);
        assert_eq!(window.to_string(), "H####\ne####\ny####\n ####\nt####");
        let q = window.queue();
        assert_eq!(q, Some(vec![
                                        Queued::from((0, 0, Cell::new('H'))),
                                        Queued::from((0, 1, Cell::new('e'))),
                                        Queued::from((0, 2, Cell::new('y'))),
                                        Queued::from((0, 0, Cell::new('H'))),
                                        Queued::from((0, 1, Cell::new('e'))),
                                        Queued::from((0, 2, Cell::new('y'))),
                                        Queued::from((0, 3, Cell::new(' '))),
                                        Queued::from((0, 4, Cell::new('t'))),
        ]));
    }
}


