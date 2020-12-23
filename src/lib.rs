/// All available colors in terminals
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Color {
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
    NONE,
}

#[cfg(feature = "with_crossterm")]
impl From<Color> for crossterm::style::Color {
    fn from(color: Color) -> crossterm::style::Color {
        match color {
            Color::Reset           => crossterm::style::Color::Reset,
            Color::Black           => crossterm::style::Color::Black,
            Color::DarkGrey        => crossterm::style::Color::DarkGrey,
            Color::Red             => crossterm::style::Color::Red,
            Color::DarkRed         => crossterm::style::Color::DarkRed,
            Color::Green           => crossterm::style::Color::Green,
            Color::DarkGreen       => crossterm::style::Color::DarkGreen,
            Color::Yellow          => crossterm::style::Color::Yellow,
            Color::DarkYellow      => crossterm::style::Color::DarkYellow,
            Color::Blue            => crossterm::style::Color::Blue,
            Color::DarkBlue        => crossterm::style::Color::DarkBlue,
            Color::Magenta         => crossterm::style::Color::Magenta,
            Color::DarkMagenta     => crossterm::style::Color::DarkMagenta,
            Color::Cyan            => crossterm::style::Color::Cyan,
            Color::DarkCyan        => crossterm::style::Color::DarkCyan,
            Color::White           => crossterm::style::Color::White,
            Color::Grey            => crossterm::style::Color::Grey,
            Color::Rgb { r, g, b } => crossterm::style::Color::Rgb { r, g, b },
            Color::AnsiValue(n)    => crossterm::style::Color::AnsiValue(n),
            Color::NONE            => crossterm::style::Color::White,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Queued {
    pub x: u16,
    pub y: u16,
    pub cells: Vec<char>,
    pub color: Vec<(Color, Color)>,
}

impl Queued {
    fn new(x: u16, y: u16, cells: Vec<char>, color: Vec<(Color, Color)>) -> Self {
        Self {
            x, y, cells, color,
        }
    }
}

/// Buffer holds char with Color's and is of a set size once built
#[derive(Debug, Clone)]
pub struct Buffer {
    width: usize,
    height: usize,
    cells: Vec<char>,
    color: Vec<(Color, Color)>,
    queue: Vec<Queued>,
}

impl Buffer {
    pub fn new(width: usize, height: usize, filler: char, fg: Color, bg: Color) -> Self {
        let total = (width * height) as usize;
        let cells = vec![filler; total];
        let color = vec![(fg, bg); total];
        let mut queue = Vec::new();
        let chunk_cells = cells.chunks(width);
        let chunk_color = color.chunks(width);
        for (i, (line, color)) in chunk_cells.zip(chunk_color).enumerate() {
            queue.push(Queued::new(0, i as u16, line.to_vec(), color.to_vec()));
        }

        Self {
            width, height, cells, color, queue,
        }
    }

    /// Inserts a string slice into Buffer
    pub fn replace_line(&mut self, x: usize, y: usize, string: &str, fg: Color, bg: Color) {
        let start = y * self.width + x;
        let total = std::cmp::min((start + string.len()) - start - x, self.width);
        let end = start + total;
        let mut slice: Vec<char> = string.chars().collect();
        let mut color = vec![(fg, bg); string.len()];
        self.queue.push(Queued::new(x as u16, y as u16, string.chars().collect(), color.clone()));
        color.as_mut_slice().swap_with_slice(&mut self.color[start..end]);
        slice.as_mut_slice().swap_with_slice(&mut self.cells[start..end]);
    }

    /// Inserts a char into Buffer
    pub fn replace_char(&mut self, x: usize, y: usize, c: char, fg: Color, bg: Color) {
        let idx = y * self.width + x;
        let _ = self.color.remove(idx);
        self.color.insert(idx, (fg.clone(), bg.clone()));
        let _ = self.cells.remove(idx);
        self.cells.insert(idx, c);
        self.queue.push(Queued::new(x as u16, y as u16, vec![c], vec![(bg, fg)]));
    }

    pub fn queue(&mut self) -> Vec<Queued> {
        let queue = self.queue.clone();
        self.queue.clear();
        queue
    }
}

