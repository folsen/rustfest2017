extern crate termion;
extern crate taxi;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{Write, stdout, stdin};

use taxi::*;

pub fn main() {
	let mut game = Game::new(true);
	let stdin = stdin();
	// This line is a bit odd, we need to call this and assign it to a variable, because that has some side effects,
	// it's ugly, but necessary, or stdin won't parse the keys without requiring <Enter> to be pressed
	let mut stdout = stdout().into_raw_mode().unwrap();

	for c in stdin.keys() {
		match c.unwrap() {
			Key::Esc => break,
			Key::Up => game.make_move(Dir::Up),
			Key::Right => game.make_move(Dir::Right),
			Key::Down => game.make_move(Dir::Down),
			Key::Left => game.make_move(Dir::Left),
			_ => {}
		}
		game.print_map();
		if game.has_won() {
			write!(stdout, "You won the game!").unwrap();
			break;
		}
	}
}
