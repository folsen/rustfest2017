extern crate termion;
extern crate roguelike;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io::{stdout, stdin};

use roguelike::*;

pub fn main() {
	let mut game = Game::new(true);
	let stdin = stdin();
	// This line is a bit odd, we need to call this and assign it to a variable, because that has some side effects,
	// it's ugly, but necessary, or stdin won't parse the keys without requiring <Enter> to be pressed
	let stdout = stdout().into_raw_mode().unwrap();

	for c in stdin.keys() {
		match c.unwrap() {
			Key::Esc => break,
			Key::Up => game.enter_move(&Dir::Up, true),
			Key::Right => game.enter_move(&Dir::Right, true),
			Key::Down => game.enter_move(&Dir::Down, true),
			Key::Left => game.enter_move(&Dir::Left, true),
			_ => false
		};
		game.print_map();
		if game.has_won() {
			break;
		}
	}
}
