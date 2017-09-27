extern crate termion;
extern crate rand;

use rand::Rng;
use std::fmt;
use std::io::{Write, stdout, stdin};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

/// Move direction
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Dir {
	Up,
	Right,
	Down,
	Left,
}

impl Dir {
	/// Converts u32 to `Dir`
	pub fn from_u32(int: u32) -> Result<Dir, &'static str> {
		match int {
			0 => Ok(Dir::Up),
			1 => Ok(Dir::Right),
			2 => Ok(Dir::Down),
			3 => Ok(Dir::Left),
			_ => Err("Cannot convert u32 to `Dir`"),
		}
	}
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Object {
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

/// Calculates distance between two points
pub fn distance(p0: (u32, u32), p1: (u32, u32)) -> f64 {
	((((p0.0 as i64 - p1.0 as i64).pow(2) + (p0.1 as i64 - p1.1 as i64).pow(2)) as f64).sqrt())
}

type World = [[Object; Game::WORLD_WIDTH]; Game::WORLD_HEIGHT];

/// The Game with accompanying state
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Game {
	/// The current state of the game board, will change when you take actions
	world: World,
	/// Position of the player, defined as (row, column) coordinate on the world map
	position: (u32, u32),
	/// Position of the passenger, defined as (row, column) coordinate on the world map
	passenger: (u32, u32),
	/// Passenger has been picked up
	picked_up: bool,
	/// Position of the goal, defined as (row, column) coordinate on the world map
	goal: (u32, u32),
	/// How many moves the player has made
	moves: u32,
}

impl Game {
	const WORLD_WIDTH: usize = 11;
	const WORLD_HEIGHT: usize = 11;

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

	/// Returns size of the game world
	pub fn world_size(&self) -> (usize, usize) {
		(Self::WORLD_HEIGHT, Self::WORLD_WIDTH)
	}

	/// Returns true if player has won the game
	pub fn has_won(&self) -> bool {
		self.position == self.goal && self.picked_up
	}

	/// Returns player position
	pub fn player_position(&self) -> (u32, u32) {
		self.position
	}

	/// Returns true if passenger has been picked up
	pub fn passenger_picked_up(&self) -> bool {
		self.picked_up
	}

	/// Returns a distance to a game goal
	pub fn distance_to_goal(&self) -> f64 {
		distance(self.goal, self.position)
	}

	/// Returns a distance to a passenger
	pub fn distance_to_passenger(&self) -> f64 {
		distance(self.passenger, self.position)
	}

	/// Enter move directly instead of starting an stdin loop, for instance from an automated player
	/// Returns a bool signifying whether or not this move lead to winning the game
	pub fn enter_move(&mut self, dir: Dir, print: bool) -> bool {
		self.make_move(dir);
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
		let mut stdout = stdout().into_raw_mode().unwrap();

		for c in stdin.keys() {
			match c.unwrap() {
				Key::Esc => break,
				Key::Up => self.make_move(Dir::Up),
				Key::Right => self.make_move(Dir::Right),
				Key::Down => self.make_move(Dir::Down),
				Key::Left => self.make_move(Dir::Left),
				_ => {}
			}
			self.print_map();
			if self.has_won() {
				write!(stdout, "You won the game!").unwrap();
				break;
			}
		}
	}

	fn make_move(&mut self, dir: Dir) {
		self.moves += 1;
		let target = match dir {
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
		for r in 0..Game::WORLD_HEIGHT {
			for c in 0..Game::WORLD_WIDTH {
				let pos = (r as u32, c as u32);
				if pos == self.position {
					write!(stdout, "{}{}", termion::color::Fg(termion::color::LightCyan), "\u{2588}").unwrap();
				} else {
					write!(stdout, "{}", self.world[r][c]).unwrap();
				}
			}
			write!(stdout, "\n{}", termion::cursor::Goto(1, (r + 2) as u16)).unwrap();
		}
		write!(stdout, "\nMoves: {}", self.moves).unwrap();
		write!(stdout,
			   "\n\n{}Press <ESC> to exit game.\n{}",
			   termion::cursor::Goto(1, 14),
			   termion::cursor::Goto(1, 15),).unwrap();
		stdout.flush().unwrap();
	}
}

fn simple_world() -> (World, (u32, u32), (u32, u32)) {
	let mut rng = rand::thread_rng();
	let w = [
		[Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Empty, Object::Wall],
		[Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall, Object::Wall],
	];
	// Difficulty of the state space increases with the variation in goals and passengers
	let goals = vec![(8,8), (1,2), (1,8), (8,1)];
	let passengers = vec![(3,4), (4,8), (6,1), (6,8)];
	(w, rng.choose(&passengers).unwrap().clone(), rng.choose(&goals).unwrap().clone())
}
