extern crate taxi;

use taxi::*;

pub fn main() {
	let mut game = Game::new(true);
	game.enter_loop()
}
