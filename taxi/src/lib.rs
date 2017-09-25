extern crate termion;
extern crate rand;

use rand::Rng;
use std::fmt;
use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

static ROW_SIZE: usize = 11;
static COL_SIZE: usize = 11;

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
	Passenger,
	Goal,
	Empty,
}

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Object::Wall => write!(f, "{}{}", termion::color::Fg(termion::color::White), "\u{2588}"),
			Object::Passenger => write!(f, "{}{}", termion::color::Fg(termion::color::Yellow), "\u{2588}"),
			Object::Goal => write!(f, "{}{}", termion::color::Fg(termion::color::Green), "\u{2588}"),
			Object::Empty => write!(f, "{}{}", termion::color::Fg(termion::color::Black), "\u{2588}"),
		}
	}
}

type World = Vec<Vec<Object>>;

/// The Game with accompanying state
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Game {
	/// The current state of the game board, will change when you take actions
	world: World,
	/// Position of the player, defined as (row, column) coordinate on the world map
	pub position: (u32, u32),
	/// Position of the player, defined as (row, column) coordinate on the world map
	pub passenger: (u32, u32),
	///
	pub picked_up: bool,
	/// Position of the player, defined as (row, column) coordinate on the world map
	pub goal: (u32, u32),
	/// How many moves the player has made
	pub moves: u32,
}

impl Game {
	/// Initialize a new game state
	pub fn new(print: bool) -> Game {
		let (w, p, g) = simple_world();
		let mut game = Game {
			world: w,
			position: (1, 1),
			passenger: p,
			picked_up: false,
			goal: g,
			moves: 0,
		};
		game.world[p.0 as usize][p.1 as usize] = Object::Passenger;
		game.world[g.0 as usize][g.1 as usize] = Object::Goal;
		if print {
			game.print_map()
		};
		game
	}

	pub fn has_won(&self) -> bool {
		self.position == self.goal && self.picked_up
	}

	pub fn distance_to_goal(&self) -> f64 {
		((((self.goal.0 as i32 - self.position.0 as i32).pow(2) + (self.goal.1 as i32 - self.position.1 as i32).pow(2)) as f64).sqrt())
	}

	pub fn distance_to_passenger(&self) -> f64 {
		((((self.passenger.0 as i32 - self.position.0 as i32).pow(2)
		+ (self.passenger.1 as i32 - self.position.1 as i32).pow(2)
		) as f64).sqrt())
	}

	/// Enter move directly instead of starting an stdin loop, for instance from an automated player
	/// Returns a bool signifying whether or not this move lead to winning the game
	pub fn enter_move(&mut self, dir: &Dir, print: bool) -> bool {
		self.make_move(&dir);
		if print {
			self.print_map();
		};
		if self.has_won() {
			true
		} else {
			false
		}
	}

	/// Enter the main game loop that prints the map and accepts input
	pub fn enter_loop(&mut self) {
		let stdin = stdin();
		// This line is a bit odd, we need to call this and assign it to a variable, because that has some side effects,
		// it's ugly, but necessary, or stdin won't parse the keys without requiring <Enter> to be pressed
		let stdout = stdout().into_raw_mode().unwrap();

		for c in stdin.keys() {
			match c.unwrap() {
				Key::Esc => break,
				Key::Up => self.make_move(&Dir::Up),
				Key::Right => self.make_move(&Dir::Right),
				Key::Down => self.make_move(&Dir::Down),
				Key::Left => self.make_move(&Dir::Left),
				_ => {}
			}
			self.print_map();
			if self.has_won() {
				println!("You won the game!");
				break;
			}
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
		match self.world[target.0 as usize][target.1 as usize] {
			Object::Wall => (),
			Object::Goal => self.position = target,
			Object::Passenger => {
				self.picked_up = true;
				self.world[target.0 as usize][target.1 as usize] = Object::Empty;
				self.position = target;
			}
			Object::Empty => {
				self.position = target;
			}
		}
	}

	pub fn print_map(&self) {
		let mut stdout = stdout().into_raw_mode().unwrap();
		write!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
		for r in 0..ROW_SIZE {
			for c in 0..COL_SIZE {
				let pos = (r as u32, c as u32);
				if pos == self.position {
					write!(stdout, "{}{}", termion::color::Fg(termion::color::LightCyan), "\u{2588}").unwrap();
				} else {
					write!(stdout, "{}", self.world[r][c]).unwrap();
				}
			}
			write!(stdout, "\n{}", termion::cursor::Goto(1, (r + 2) as u16)).unwrap();
		}
		print!("\nMoves: {}", self.moves);
		write!(stdout,
			   "\n\n{}Press <ESC> to exit game.\n{}",
			   termion::cursor::Goto(1, 14),
			   termion::cursor::Goto(1, 15),).unwrap();
		stdout.flush().unwrap();
	}
}

fn simple_world() -> (World, (u32, u32), (u32, u32)) {
	let mut rng = rand::thread_rng();
	let w = vec![
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		vec![Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
	];
	// Difficulty of the state space increases with the variation in goals and passengers
	let goals = vec![(8,8), (1,2), (1,8), (8,1)];
	let passengers = vec![(3,4), (4,8), (6,1), (6,8)];
	(w, rng.choose(&passengers).unwrap().clone(), rng.choose(&goals).unwrap().clone())
}
