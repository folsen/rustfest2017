extern crate rurel;
extern crate rand;
extern crate bejeweled;

use bejeweled::*;

use rand::{thread_rng, Rng};
use rurel::AgentTrainer;
use rurel::mdp::State;
use rurel::mdp::Agent;
use rurel::strategy::learn::QLearning;
use rurel::strategy::explore::RandomExploration;
use rurel::strategy::terminate::FixedIterations;

#[derive(PartialEq, Eq, Hash, Clone)]
struct GameState {
	game: Game,
}

impl State for GameState {
	type A = Move;
	fn reward(&self) -> f64 {
		(self.game.score - self.game.moves) as f64
	}
	fn actions(&self) -> Vec<Move> {
		all_valid(&self.game)
	}
}

struct GameAgent { state: GameState }

impl Agent<GameState> for GameAgent {
	fn current_state(&self) -> &GameState {
		&self.state
	}
	fn take_action(&mut self, action: &Move) -> () {
		let prev_score = self.state.game.score;
		self.state.game.make_move(action);
		if prev_score == self.state.game.score {
			print!("r");
			self.state.game = Game::new(false);
		}
	}
}

fn all_moves() -> Vec<Move> {
	let mut moves = Vec::new();
	// All possible horizontal moves
	for r in 0..8 {
		for c in 0..7 {
			moves.push(Move { row1: r, col1: c, row2: r, col2: c+1 });
		}
	}
	// All possible vertical moves
	for r in 0..7 {
		for c in 0..8 {
			moves.push(Move { row1: r, col1: c, row2: r+1, col2: c });
		}
	}
	moves
}

fn all_valid(game: &Game) -> Vec<Move> {
	let mut scoring_moves = Vec::new();
	let mut tmp_game = game.clone();
	for mov in all_moves() {
		tmp_game.execute_move(&mov);
		if tmp_game.pieces_to_remove().len() > 0 {
			scoring_moves.push(mov.clone());
		}
		tmp_game.execute_move(&mov); // Move back
	}
	if scoring_moves.len() == 0 {
		all_moves()
	} else {
		scoring_moves
	}
}

fn main() {
	println!("Training Agent");
	let mut trainer = AgentTrainer::new();
	let mut agent = GameAgent { state: GameState { game: Game::new(false) }};
	trainer.train(&mut agent,
				&QLearning::new(0.2, 0.01, 2.),
				&mut FixedIterations::new(1_000_000),
				&RandomExploration::new());

	println!("Playing a game with trained agent");
	let mut game_state = GameState { game: Game::new(true) };
	loop {
		let valids = all_valid(&game_state.game);
		let random_move = thread_rng().choose(&valids).unwrap_or(&valids[0]);
		let mov = if let Some(good_moves) = trainer.expected_values(&game_state) {
			println!("Found some good moves: {:?}", good_moves);
			good_moves.into_iter().max_by_key(|&(_, val)| *val as i64).unwrap_or((random_move, &0.0)).0
		} else {
			print!(".");
			random_move
		};
		if !game_state.game.make_move(&mov) {
			println!("Game over");
			game_state.game.print_board();
			break;
		}
	}
}
