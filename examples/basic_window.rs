use std::io::{Write, stdout};
use crossterm;
use buffy;

fn main() {
    let mut write = stdout();
    let mut buff = buffy::Buffer::new(50, 30);
    let mut hline = Vec::new();
    for c in "Line changed".chars() {
        hline.push(buffy::Cell::new(c));
    }

    let vline = buffy::Line::from("This is a line struct");


    buff.insert_line(3, &mut hline);
    buff.insert_vline(20, &mut vline.as_slice());
    buff.draw(&mut |lines| {
        for (idx, line) in lines.split("\n").enumerate() {
            crossterm::queue!(
                write,
                crossterm::cursor::MoveTo(0, idx as u16),
                crossterm::style::Print(line))
                    .expect("Failed to print to screen.");
        }
        write.flush().expect("Failed to flush");
    });

    println!("dklsajkjfsa {}", buff);
}
