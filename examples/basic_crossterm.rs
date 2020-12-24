use buffy::{Buffer, Color};
use crossterm::{queue, Result, style, terminal, cursor};
use std::io::{stdout, Write};

fn main() -> Result<()> {
    let mut writer = stdout();
    let (w, h) = terminal::size()?;
    let (w, h) = (w as usize, h as usize);
    let mut buf = Buffer::new(w, h-1, ' ', Color::White, Color::Rgb{r: 0, g: 134, b: 134});
    let text = "CrossTerm is awesome!";
    buf.replace_char(w/2, h/3, '|', Color::Red, Color::Cyan);
    buf.replace_line(w/2 - text.len()/2, h/2, text, Color::White, Color::Green);
    queue!(&mut writer, terminal::Clear(terminal::ClearType::All), cursor::SavePosition)?;
    for q in buf.queue() {
        for (i, c) in q.cells.iter().enumerate() {
            queue!(
                &mut writer,
                cursor::MoveTo(q.x + i as u16, q.y),
                style::SetColors(style::Colors::new(q.color[i].0.into(), q.color[i].1.into())),
                style::Print(c)
            )?;
        }
    }
    queue!(&mut writer, cursor::RestorePosition)?;
    Ok(())
}
