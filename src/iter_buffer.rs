use crate::{Cell, Buffer};

pub struct IterBuffer<'a> {
    pub(crate) inner: &'a Buffer,
    pub(crate) index: usize,
}

impl<'a> Iterator for IterBuffer<'a> {
    type Item = &'a Cell;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        self.inner.cells.get(index)
    }
}


pub struct IterMutBuffer<'a> {
    pub(crate) inner: &'a mut Buffer,
    pub(crate) index: usize,
}

impl<'a> Iterator for IterMutBuffer<'a> {
    type Item = &'a mut Cell;

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        if i < self.inner.cells.len() {
            let ptr = self.inner.cells.as_mut_ptr();

            // Safety: No clue.

            unsafe {
                Some(&mut *ptr.add(i))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iter_buffer() {
        let buffer = Buffer::new(10, 10, '#');
        for (i, c) in buffer.iter().enumerate() {
            if i % buffer.width == 0 && i != 0 {
                eprint!("{}\n", c);
            } else {
                eprint!("{}", c);
            }
        }
    }

    #[test]
    fn test_iter_mut_buffer() {
        let mut buffer = Buffer::new(5, 2, '#');
        for (i, c) in buffer.iter_mut().enumerate() {
            *c = Cell::from(((i + 48) as u8) as char);
        }
        assert_eq!(buffer.to_string(), "01234\n56789".to_string());
    }
}
