extern crate id_tree as tree;
extern crate roguelike;

use roguelike::*;
use tree::*;
use tree::InsertBehavior::*;
use tree::RemoveBehavior::*;

static MAX_LEVELS: u32 = 13; // Maximum depth of tree
static MOVE_STREAK: u32 = 5; // How many moves in a row it takes before adding new layers to the tree

fn main() {
	/*
	See https://docs.rs/id_tree/1.1.3/id_tree/index.html for documentation on how to use the tree library

	1. Create a decision-tree with `MAX_LEVELS` depth, and then start a game loop
	2. Create a game with Game::new(false) to not print the board and Game::new(true) to print it
	3. Use game.enter_move(move, true/false) to put in a move and optionally print it
	4. Sleep a bit so you can see the move on the screen
	5. Expand the decision tree, you can optionally make use of the strategy of moving a few steps without expanding the tree
	*/
}
