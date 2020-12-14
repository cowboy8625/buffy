use crossterm::style::{style, Color, Attribute};
use std::io::{Write, stdout};
use crossterm;
use buffy;

fn main() {
    let string = "No color on this line.";

    let styled = style("This line has color")
            .with(Color::Yellow)
            .on(Color::Blue)
            .attribute(Attribute::Bold);
    let mut write = stdout();
    let mut buff = buffy::Buffer::new(50, 30, ' ');
    let hline = buffy::Line::from(string);
    let vline = buffy::Line::from(&styled.to_string()[..]);


    buff.insert_vline(&(20u16, 4u16).into(), &vline);
    buff.insert_line(&(0u16, 3u16).into(), hline);
    crossterm::queue!(write, crossterm::terminal::Clear(crossterm::terminal::ClearType::All)).expect("Couldnt Clear");
    let mut my = 0;
    if let Some(qline) = buff.queue() {
        qline.iter().for_each(|line| {
            let (x, y) = line.cords();
            my = std::cmp::max(my, y);
            crossterm::queue!(
                write,
                crossterm::cursor::MoveTo(x, y),
                crossterm::style::Print(line.to_string()))
                    .expect("Failed to print to screen.");
            crossterm::queue!(write, crossterm::cursor::MoveTo(0, my + 1))
                .expect("Couldnt move cursor");
        });
    }
    write.flush()
        .expect("Failed to flush");
    // let thing = crossterm::style::SetForegroundColor(Color::Rgb { r: 0, g: 139, b: 139 });
    // println!("{}", thing);
}
