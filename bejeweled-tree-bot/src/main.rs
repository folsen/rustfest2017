extern crate indextree;
extern crate bejeweled;

use bejeweled::*;

use indextree::{Arena, NodeId};

fn main() {
	let mut game = Game::new(true);
	loop {
		let maybe_move = find_best_move(&game);
		if let Some(mov) = maybe_move {
			println!("Making move {:?}", mov);
			game.make_move(&mov);
		} else {
			println!("Game over! Couldn't find any more moves or time is up. Final board is:");
			game.print_board();
			break;
		}
	}
}

fn find_best_move(game: &Game) -> Option<Move> {
	let mut arena = Arena::new();
	let top_node = arena.new_node((Move { row1: 0, col1: 0, row2: 0, col2: 0}, game.clone(), 0));
	println!("Populating arena with possible moves");
	populate_arena_for_game(&mut arena, &top_node, 0);

	println!("Finding best move");
	match find_best_branch(&top_node, &arena) {
		Some((node_id, _)) => Some(arena[node_id].data.0.clone()),
		None => None
	}
}

type Step = (Move, Game, i32);

fn find_best_branch(top_node: &NodeId, arena: &Arena<Step>) -> Option<(NodeId, i32)> {
	top_node.children(&arena).fold(None, |top_scoring, child_id| {
		let node_score = score_node(&child_id, arena);
		if let Some((top_node, top_score)) = top_scoring {
			if top_score < node_score {
				Some((child_id, node_score))
			} else {
				Some((top_node, top_score))
			}
		} else if node_score > 0 {
			Some((child_id, node_score))
		} else {
			None
		}
	})
}

fn score_node(node_id: &NodeId, arena: &Arena<Step>) -> i32 {
	let score = arena[*node_id].data.2;
	let max_child = node_id.children(arena).max_by_key(|child| arena[*child].data.2 );
	if let Some(child) = max_child {
		score + score_node(&child, arena)
	} else {
		score
	}
}

fn populate_arena_for_game(mut arena: &mut Arena<Step>, parent_node: &NodeId, depth: u32) {
	for mov in all_moves() {
		let mut node_game = arena[*parent_node].data.1.clone();
		let prev_score = node_game.score;
		node_game.make_move(&mov);
		if node_game.score > prev_score {
			let move_score = node_game.score - prev_score;
			let valid_move = arena.new_node((mov, node_game, move_score));
			parent_node.append(valid_move, arena);
			if depth < 2 {
				populate_arena_for_game(&mut arena, &valid_move, depth + 1)
			}
		}
	}
}

/// This could be a static list of all possible moves on the board
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
