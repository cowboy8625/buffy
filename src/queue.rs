use crate::{Line, Cell};

#[derive(Debug, Clone)]
pub enum Queueable {
    Cell(Cell),
    Line(Line),
}

impl Queueable {
    pub fn value(self) -> Line {
        match self {
            Self::Cell(c) => Line::from(vec![c].as_slice()),
            Self::Line(l) => l,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queued {
    x: u16,
    y: u16,
    inner: Queueable,
}

impl Queued {
    pub fn end(&self) -> u16 {
        match &self.inner {
            Queueable::Cell(_) => 1 + self.x,
            Queueable::Line(line) => line.len() as u16 + self.x,
        }
    }

    pub fn cords(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn value(self) -> Line {
        self.inner.value()
    }
}

impl From<(u16, u16, Cell)> for Queued {
    fn from((x, y, cell): (u16, u16, Cell)) -> Self {
        Self {
            x, y , inner: Queueable::Cell(cell),
        }
    }
}

impl From<(u16, u16, Line)> for Queued {
    fn from((x, y, line): (u16, u16, Line)) -> Self {
        Self {
            x, y , inner: Queueable::Line(line),
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_enum_queueable() {
        let q = Queueable::Cell(Cell::new('b'));
        assert_eq!(q.value(), Line::from("b"));
    }
}




