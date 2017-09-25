extern crate bejeweled;

use std::io;
use bejeweled::*;

pub fn main() {
	let mut game = Game::new(true);
	game.print_board();
	loop {
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(_) => {
				let mut iter = input.split_whitespace();
				let mov = Move {
					row1: iter.next().map(str::parse).unwrap_or(Ok(0)).unwrap_or(0),
					col1: iter.next().map(str::parse).unwrap_or(Ok(0)).unwrap_or(0),
					row2: iter.next().map(str::parse).unwrap_or(Ok(0)).unwrap_or(0),
					col2: iter.next().map(str::parse).unwrap_or(Ok(0)).unwrap_or(0),
				};
				game.make_move(&mov);
				game.print_board();
			}
			Err(error) => println!("Couldn't read input: {}", error),
		}
	}
}
