extern crate renforce as re;
extern crate roguelike;

use roguelike::*;

use re::environment::{Environment, Observation};
use re::environment::Finite;

use re::trainer::OnlineTrainer;
use re::trainer::QLearner;

use re::agent::Agent;
use re::agent::qagents::EGreedyQAgent;

use re::util::TimePeriod;
use re::util::table::QTable;
use re::util::chooser::Uniform;

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
		if self.state.action == Action::Won { 1. } else { 0. }
	}
}

impl Environment for GameState {
	type State = ((Finite, Finite), Vec<Vec<Finite>>);
	type Action = Finite;

	fn state_space(&self) -> Self::State {
		((Finite::new(24), Finite::new(38)), vec![vec![Finite::new(6)]])
	}
	fn action_space(&self) -> Finite {
		// The agent has 4 actions: move {up, down, left, right}
		Finite::new(4)
	}
	fn step(&mut self, action: &u32) -> Observation<Self::State> {
		self.state.enter_move(&Dir::from_u32(action), false);
		let done = self.state.action == Action::Won;
		Observation {
			state: self.state.to_int_state(),
			// Punish agent for every step it takes, but reward it when it reaches the goal
			// The optimal strategy is then to take the shortest path to the goal
			reward: self.reward(),
			done: done
		}
	}
	fn reset(&mut self) -> Observation<Self::State> {
		self.state = Game::new(false);
		Observation {
			state: self.state.to_int_state(),
			reward: self.reward(),
			done: false
		}
	}
	fn render(&self) {
		self.state.print_map();
	}
}

fn main() {
	let mut env = GameState::new();

	// The agent will use a table as its Q-function
	let q_func = QTable::new();
	// Creates an epsilon greedy Q-agent
	// Agent will use softmax to act randomly 5% of the time
	let mut agent = EGreedyQAgent::new(q_func, env.action_space(), 0.9, Uniform);
	// We will use Q-learning to train the agent with
	// discount factor and learning rate both 0.9 and
	// 10000 training iterations
	let mut trainer = QLearner::new(env.action_space(), 0.9, 0.9, TimePeriod::TIMESTEPS(1_000_000));

	// Magic happens
	trainer.train(&mut agent, &mut env);

	// Simulate one episode of the environment to see what the agent learned
	let mut obs = env.reset();
	agent.set_epsilon(0.1);
	while !obs.done {
		env.render();

		let action = agent.get_action(&obs.state);
		obs = env.step(&action);

		std::thread::sleep(std::time::Duration::from_millis(200));
	}
	env.render();
}
