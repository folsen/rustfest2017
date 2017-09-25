extern crate roguelike;

use roguelike::*;

pub fn main() {
	let mut game = Game::new(true);
	game.enter_loop()
}
