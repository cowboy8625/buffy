///! # Buffy
mod buffer;
mod cell;
mod iter_buffer;
mod line;
mod location;
mod queue;

pub use buffer::Buffer;
pub use cell::Cell;
pub use iter_buffer::{IterBuffer, IterMutBuffer};
pub use line::Line;
pub use location::Location;
pub use queue::{Queueable, Queued};

pub fn strip_code(string: &str) -> (Option<String>, String, Option<String>) {
    let raw = regex::Regex::new("\u{1b}\\[[\\d;]+m")
        .expect("Regex Failed to parse string.")
        .replace_all(string.clone(), "");
    let result: Vec<_> = string.clone().split(&raw.to_string()).collect();
    let start = result
        .get(0)
        .map(std::string::ToString::to_string)
        .filter(|i| !i.is_empty());
    let end = result
        .get(1)
        .map(std::string::ToString::to_string)
        .filter(|i| !i.is_empty());
    (start, raw.to_string(), end)
}
