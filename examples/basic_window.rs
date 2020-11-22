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
    let mut buff = buffy::Buffer::new(50, 30);
    let mut hline = buffy::Line::from(string);
    let vline = buffy::Line::from(&styled.to_string()[..]);


    buff.insert_line(3, hline.as_mut_slice());
    buff.insert_vline(20, vline.as_slice());
    buff.get(&mut |lines| {
        for (idx, line) in lines.split("\n").enumerate() {
            crossterm::queue!(
                write,
                crossterm::cursor::MoveTo(0, idx as u16),
                crossterm::style::Print(line))
                    .expect("Failed to print to screen.");
        }
        write.flush().expect("Failed to flush");
    });
}
