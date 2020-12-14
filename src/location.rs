pub enum Location {
    Cords{ x: u16, y: u16 },
    Index(usize),
}

impl From<(usize, usize)> for Location {
    fn from((x, y): (usize, usize)) -> Self {
        Self::Cords { x: x as u16, y: y as u16 }
    }
}

impl From<(u16, u16)> for Location {
    fn from((x, y): (u16, u16)) -> Self {
        Self::Cords { x, y }
    }
}

impl From<usize> for Location {
    fn from(idx: usize) -> Self {
        Self::Index(idx)
    }
}

impl From<u16> for Location {
    fn from(idx: u16) -> Self {
        Self::Index(idx as usize)
    }
}
