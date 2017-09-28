extern crate ansi_term;

use std::fmt;
use std::io::{Write, stdout};

use ansi_term::Colour::{White, Red, Yellow, Blue, Green, Black, Cyan};

static GOLD_VALUE: u32 = 30;
static ENEMY_VALUE: u32 = 20;
static ROW_SIZE: usize = 19;
static COL_SIZE: usize = 30;

fn clear_screen() {
	std::io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap()
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}

impl Dir {
	pub fn from_u32(int: &u32) -> Dir {
		match *int {
			0 => Dir::Up,
			1 => Dir::Right,
			2 => Dir::Down,
			_ => Dir::Left,
		}
	}
}

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

type World = Vec<Vec<Object>>;

/// The Game with accompanying state
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Game {
	/// The current state of the game board, will change when you take actions
	world: World,
	/// Position of the player, defined as (row, column) coordinate on the world map
	pub position: (usize, usize),
	/// How many moves the player has made
	pub moves: u32,
	/// What score the player has so far
	pub score: u32,
	/// Last action that you took
	pub action: Action,
	/// Whether or not the player has picked up the sword
	has_sword: bool,
}

impl Game {
	/// Initialize a new game state
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
	pub fn to_int_state(&self) -> ((u32, u32), Vec<Vec<u32>>) {
		((self.position.0 as u32, self.position.1 as u32), self.world_to_ints())
	}

	pub fn get_moves(&self) -> u32 {
		self.moves
	}

	pub fn get_score(&self) -> u32 {
		self.score
	}

	pub fn distance_to_goal(&self) -> f64 {
		((((1 - self.position.0 as i32).pow(2) + (21 - self.position.1 as i32).pow(2)) as f64).sqrt())
	}

	pub fn has_won(&self) -> bool {
		self.action == Action::Won
	}

	fn world_to_ints(&self) -> Vec<Vec<u32>> {
		self.world.iter().map(|cols| cols.iter().map(|x| {
			match *x {
				Object::Wall  => 0,
				Object::Enemy => 1,
				Object::Gold  => 2,
				Object::Sword => 3,
				Object::Goal  => 4,
				Object::Empty => 5,
			}
		}).collect()).collect()
	}

	/// Enter move directly instead of starting an stdin loop, for instance from an automated player
	/// Returns a bool signifying whether or not this move lead to winning the game
	pub fn enter_move(&mut self, dir: &Dir, print: bool) -> bool {
		self.make_move(&dir);
		if print {
			self.print_map();
		};
		if self.action == Action::Won {
			true
		} else {
			false
		}
	}

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

	fn move_into(&mut self, target: (usize, usize)) {
		self.world[target.0][target.1] = Object::Empty;
		self.position = target;
	}

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
