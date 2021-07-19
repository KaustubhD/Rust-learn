use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use R7_invaders::frame::{Drawable, new_frame};
use R7_invaders::player::Player;
use R7_invaders::{frame, render};
use crossterm::event::{Event, KeyCode};
use rusty_audio::Audio;
use std::{io, thread};
use crossterm::{ExecutableCommand, event, terminal};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
	let mut audio = Audio::new();
	audio.add("explode", "sounds/explode.wav");
	audio.add("lose", "sounds/lose.wav");
	audio.add("move", "sounds/move.wav");
	audio.add("pew", "sounds/pew.wav");
	audio.add("startup", "sounds/startup.wav");
	audio.add("win", "sounds/win.wav");

	audio.play("startup");

	let mut stdout = io::stdout();
	terminal::enable_raw_mode()?;
	stdout.execute(EnterAlternateScreen)?;
	stdout.execute(Hide)?;


	let (render_tx, render_rx) = mpsc::channel();
	let render_handle = thread::spawn(move || {
		let mut last_frame = frame::new_frame();
		let mut stdout = io::stdout();
		render::render(&mut stdout, &last_frame, &last_frame, true);
		loop {
			let curr_frame = match render_rx.recv() {
				Ok(x) => x,
				Err(_) => break,
			};
			render::render(&mut stdout, &last_frame, &curr_frame, false);
			last_frame = curr_frame;
		}
	});

	let mut player = Player::new();
	let mut instant = Instant::now();
	'gameloop: loop {

		let delta = instant.elapsed();
		instant = Instant::now();
		let mut curr_frame = new_frame();

		while event::poll(Duration:: default())? {
			if let Event::Key(key_event) = event::read()? {
				match key_event.code {
					KeyCode::Left => player.move_left(),
					KeyCode::Right => player.move_right(),
					KeyCode::Char(' ') | KeyCode::Enter => {
						if player.shoot() {
							audio.play("pew");
						}
					}
					KeyCode::Esc | KeyCode::Char('q') => {
						audio.play("lose");
						break 'gameloop;
					}
					_ => {
						
					}
				}
			}
		}

		player.update_timer(delta);

		player.draw(&mut curr_frame);
		let _ = render_tx.send(curr_frame);
		thread::sleep(Duration::from_millis(1));
	}

	drop(render_tx);
	render_handle.join().unwrap();
	audio.wait();
	stdout.execute(Show)?;
	stdout.execute(LeaveAlternateScreen)?;
	terminal::disable_raw_mode()?;
	Ok(())
}
