use crate::{Line, Cell};

#[derive(Debug, Clone)]
pub enum Queueable {
    Cell(Cell),
    Line(Line),
}

#[derive(Debug, Clone)]
pub struct Queued {
    x: u16,
    y: u16,
    inner: Queueable,
}

impl Queued {
    pub fn end(&self) -> u16 {
        match self.inner {
            Queueable::Cell(_) => 1 + self.x,
            Queueable::Line(line) => line.len() as u16 + self.x,
        }
    }

    pub fn cords(&self) -> (u16, u16) {
        (self.x, self.y)
    }

    pub fn value(&self) -> &Queueable {
        &self.inner
    }
}
