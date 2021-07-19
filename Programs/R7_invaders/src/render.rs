use std::io::{Stdout, Write};

use crossterm::{QueueableCommand, cursor::MoveTo, style::{Color, SetBackgroundColor}, terminal::{Clear, ClearType}};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
	if force {
		stdout.queue(SetBackgroundColor(Color::Cyan)).unwrap();
		stdout.queue(Clear(ClearType::All)).unwrap();
		stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
	}
	for (y, row) in curr_frame.iter().enumerate() {
		for (x, col) in row.iter().enumerate() {
			if *col != last_frame[y][x] || force{
				stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
				println!("{}", *col);
			}
		}
	}
	stdout.flush().unwrap();
}