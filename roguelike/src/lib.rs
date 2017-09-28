extern crate ansi_term;

use std::fmt;
use std::io::{Write, stdout};

use ansi_term::Colour::{White, Red, Yellow, Blue, Green, Black, Cyan};

static GOLD_VALUE: u32 = 30;
static ENEMY_VALUE: u32 = 20;
static ROW_SIZE: usize = 19;
static COL_SIZE: usize = 30;

/// Helper function to clear the terminal screen, not tested on Windows
fn clear_screen() {
	std::io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap()
}

/// Dir is an enum representing directions one could make a move in
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}

impl Dir {
	/// The REnforce library generates random u32's and needs to go from that to an action, in this case a Dir
	pub fn from_u32(int: &u32) -> Dir {
		match *int {
			0 => Dir::Up,
			1 => Dir::Right,
			2 => Dir::Down,
			_ => Dir::Left,
		}
	}
}

/// Object representing things on the map.
/// An enemy needs to be hit twice in a row unless you have a sword,
/// in which case they only need to be hit once in order to die.
/// Gold is just extra points and takes no effort to pick up and reaching the goal
/// finishes the game regardless of how many enemies you've killed.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Object {
	Wall,
	Enemy,
	Gold,
	Sword,
	Goal,
	Empty,
}

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Object::Wall  => write!(f, "{}", White.paint("\u{2588}")),
			Object::Enemy => write!(f, "{}", Red.paint("\u{2588}")),
			Object::Gold  => write!(f, "{}", Yellow.paint("\u{2588}")),
			Object::Sword => write!(f, "{}", Blue.paint("\u{2588}")),
			Object::Goal  => write!(f, "{}", Green.paint("\u{2588}")),
			Object::Empty => write!(f, "{}", Black.paint("\u{2588}")),
		}
	}
}

/// An Action represents something that happened, meta-data about the last move if you will.
/// You could've picked something up, attacked or killed an enemy, walked into a wall or won.
/// Nothing represents that you moved, but nothing happened, meaning you moved into an empty square.
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Action {
	PickedSword(usize, usize),
	PickedGold(usize, usize),
	KilledEnemy(usize, usize),
	AttackedEnemy(usize, usize),
	Nothing,
	Won,
	WalkedIntoWall,
}

impl fmt::Display for Action {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = match *self {
			Action::PickedSword(_, _) => "You picked up a sword!",
			Action::PickedGold(_, _) => "You found some gold, that sweet sweet loot!",
			Action::KilledEnemy(_, _) => "You killed an enemy.",
			Action::AttackedEnemy(_, _) => "You attacked an enemy, but he didn't die.",
			Action::Nothing => "You're not doing anything, get moving!",
			Action::Won => "You won the game, woop!",
			Action::WalkedIntoWall => "You walked into a wall, doh.",
		};
		write!(f, "{}", s)
	}
}

/// Simple type alias for the world, a matrix of objects
type World = Vec<Vec<Object>>;

/// The Game with accompanying state
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Game {
	/// The current state of the game board, will change when you take actions
	world: World,
	/// Position of the player, defined as (row, column) coordinate on the world map
	pub position: (usize, usize),
	/// How many moves the player has made
	moves: u32,
	/// What score the player has so far
	score: u32,
	/// Last action that you took
	pub action: Action,
	/// Whether or not the player has picked up the sword
	has_sword: bool,
}

impl Game {
	/// Initialize a new game state and optionally print it
	pub fn new(print: bool) -> Game {
		let g = Game {
			world: world(),
			position: (17, 4),
			moves: 0,
			score: 0,
			action: Action::Nothing,
			has_sword: false,
		};
		if print {
			g.print_map()
		};
		g
	}

	/// Getter for the count of moves made
	pub fn get_moves(&self) -> u32 {
		self.moves
	}

	/// Getter for the current score
	pub fn get_score(&self) -> u32 {
		self.score
	}

	/// Getter for whether or not the game has been won
	pub fn has_won(&self) -> bool {
		self.action == Action::Won
	}

	/// Enter move and optionally print the map, returning whether or not this move won the game
	pub fn enter_move(&mut self, dir: &Dir, print: bool) -> bool {
		self.make_move(&dir);
		if print {
			self.print_map();
		}
		if self.has_won() {
			true
		} else {
			false
		}
	}

	/// Make a move, this just mutates the board according to the game rules
	fn make_move(&mut self, dir: &Dir) {
		self.moves += 1;
		let target = match *dir {
			Dir::Up => (self.position.0 - 1, self.position.1),
			Dir::Right => (self.position.0, self.position.1 + 1),
			Dir::Down => (self.position.0 + 1, self.position.1),
			Dir::Left => (self.position.0, self.position.1 - 1)
		};
		match self.world[target.0][target.1] {
			Object::Wall => self.action = Action::WalkedIntoWall,
			Object::Enemy => {
				if self.has_sword || self.action == Action::AttackedEnemy(target.0, target.1) {
					self.move_into(target);
					self.action = Action::KilledEnemy(target.0, target.1);
					self.score += ENEMY_VALUE;
				} else {
					self.action = Action::AttackedEnemy(target.0, target.1);
				}
			}
			Object::Gold => {
				self.move_into(target);
				self.score += GOLD_VALUE;
				self.action = Action::PickedGold(target.0, target.1);
			}
			Object::Sword => {
				self.move_into(target);
				self.has_sword = true;
				self.action = Action::PickedSword(target.0, target.1);
			}
			Object::Goal => {
				self.move_into(target);
				self.action = Action::Won;
			}
			Object::Empty => {
				self.action = Action::Nothing;
				self.position = target;
			}
		}
	}

	/// Convenience function to make a target position empty and move into it with the player
	fn move_into(&mut self, target: (usize, usize)) {
		self.world[target.0][target.1] = Object::Empty;
		self.position = target;
	}

	/// Prints the map on screen
	pub fn print_map(&self) {
		clear_screen();
		for r in 0..ROW_SIZE {
			for c in 0..COL_SIZE {
				if (r, c) == self.position {
					print!("{}", Cyan.paint("\u{2588}"));
				} else {
					print!("{}", self.world[r][c]);
				}
			}
			print!("\n\r");
		}
		print!("\n\rMoves: {} | Score: {}", self.moves, self.score);
		if self.action != Action::Nothing {
			print!(" | {}", self.action);
		}
		print!("\n\r\n\rPress <ESC> to exit game.\n\r");
		stdout().flush().unwrap();
	}
}

/*
wwwwwwwwwwwwwwwwwwwwwwwwwwwwww
wwwwwwwwwwwwwwwwww___h__wwwwww
wwwwwwwwwwwwwwwwww______wwwwww
wwwwwwwwww____e_______e_wwwwww
wwwwwwwww___e______e____wwwwww
wwwwwwwwww______e_____wwwwwwww
wwwwwwwwwww__________wwwwwwwww
wwwwwwwwwwwwwww__wwwwwwwwwwwww
wwwwwwwwwwwwwwwe_wwwwwwwwwwwww
wwwwwwwwwwwwwww__wwwwwwwwwwwww
wwwwwwwwwww_____e_______wwwwww
ww_____wwww__wwwwwwwwwe_wwwwww
w___e___www__wwwwwwwww__wwwwww
w_g________e_wwwwwwwww____e__w
ww___e__www__wwwwwwwww__e____w
wwwwwwwwwww__wwwwwwwwwwww__www
w____________wwwwwwwwww______w
w______wwwwwwwwwwwwwwww__s___w
wwwwwwwwwwwwwwwwwwwwwwwwwwwwww
*/
/// The basic world we're playing in
fn world() -> World {
	vec![
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Goal, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Enemy, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Enemy, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Gold, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Enemy, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Empty, Object::Empty, Object::Sword, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall]
	]
}
