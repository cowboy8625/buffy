use buffy::{Buffer, Color};
use crossterm::{queue, Result, style, terminal, cursor};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let mut w = stdout();
    let mut buf = Buffer::new(100, 40, ' ', Color::White, Color::Rgb{r: 0, g: 134, b: 134});
    buf.replace_char(2, 2, '|', Color::Red, Color::Cyan);
    buf.replace_line(0, 3, "Hey There!", Color::White, Color::Black);
    queue!(&mut w, terminal::Clear(terminal::ClearType::All), cursor::SavePosition)?;
    for q in buf.queue() {
        for (i, c) in q.cells.iter().enumerate() {
            queue!(
                &mut w,
                cursor::MoveTo(q.x + i as u16, q.y),
                style::SetColors(style::Colors::new(q.color[i].0.into(), q.color[i].1.into())),
                style::Print(c)
            )?;
        }
    }
    queue!(&mut w, cursor::RestorePosition)?;
    Ok(())
}
