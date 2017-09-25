extern crate renforce as re;
extern crate taxi;

use taxi::*;

use re::environment::{Environment, Observation};
use re::environment::Finite;

use re::trainer::OnlineTrainer;
use re::trainer::QLearner;

use re::agent::Agent;
use re::agent::qagents::EGreedyQAgent;

use re::util::TimePeriod;
use re::util::table::QTable;
use re::util::chooser::Uniform;

/// We need to wrap the game since we can't create an `impl Environment` for something from a different crate
/// But it also adds some convenience because we can add an `impl GameState` to add some custom logic
struct GameState {
	state: Game
}
impl GameState {
	pub fn new() -> GameState {
		GameState {
			state: Game::new(false)
		}
	}

	pub fn reward(&self) -> f64 {
		unimplemented!()
	}

	pub fn q_state(&self) -> ?? {
		unimplemented!()
	}
}

impl Environment for GameState {
	type State = ??;
	type Action = Finite;

	fn state_space(&self) -> Self::State {
		unimplemented!()
	}
	fn action_space(&self) -> Finite {
		// The agent has 4 actions: move {up, down, left, right}
		Finite::new(4)
	}
	fn step(&mut self, action: &u32) -> Observation<Self::State> {
		unimplemented!()
	}
	fn reset(&mut self) -> Observation<Self::State> {
		unimplemented!()
	}
	fn render(&self) {
		unimplemented!()
	}
}

/// You're highly encouraged to play with the parameters here, but I will provide some standard basic ones
/// Changing the randomless during learning and the iterations will have the biggest effect,
/// but changing learning and discount rates will also have a more subtle impact
fn main() {
	let mut env = GameState::new();
	// The agent will use a table as its Q-function
	let q_func = QTable::new();
	// Creates an epsilon greedy Q-agent
	// Agent will act randomly 75% of the time in a Uniform distribution during training, more or less being "random exploration"
	let mut agent = EGreedyQAgent::new(q_func, env.action_space(), 0.75, Uniform);
	// We will use Q-learning to train the agent with
	// discount factor and learning rate both 0.9 and
	// 1 000 000 training iterations
	let mut trainer = QLearner::new(env.action_space(), 0.9, 0.9, TimePeriod::TIMESTEPS(1_000_000));

	// Magic happens
	trainer.train(&mut agent, &mut env);

	// Change epsilon to act randomly 5% of the time (to avoid the bot getting stuck in a loop)
	// And run a game on-screen
	let mut obs = env.reset();
	agent.set_epsilon(0.05);
	loop {
		while !obs.done {
			env.render();
			let action = agent.get_action(&obs.state);
			obs = env.step(&action);
			std::thread::sleep(std::time::Duration::from_millis(100));
		}
		// We're done with the game here, so we could stop, or we could play another game.
		// Playing another game will be more insteresting since the game has some random state each time
		env.render();
		obs = env.reset();
		std::thread::sleep(std::time::Duration::from_millis(500));
	}
}
