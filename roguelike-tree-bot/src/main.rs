extern crate id_tree as tree;
extern crate roguelike;

use roguelike::*;
use tree::*;
use tree::InsertBehavior::*;
use tree::RemoveBehavior::*;

static MAX_LEVELS: u32 = 13; // Maximum depth of tree
static MOVE_STREAK: u32 = 5; // How many moves in a row it takes before adding new layers to the tree

fn main() {
	let mut game = Game::new(false);
	let (mut node, mut tree) = build_decision_tree(&game);
	game = Game::new(true);
	loop {
		if let Some((best, node_id)) = optimal_step(&node, &tree) {
			game.enter_move(&best, true);
			println!("Moved {:?}", best);
			prune(&node, &node_id, &mut tree);
			if game.moves % MOVE_STREAK == 0 {
				add_layer(&node_id, &mut tree);
			}
			node = node_id;
			if game.action == Action::Won {
				break;
			}
			std::thread::sleep(std::time::Duration::from_millis(300));
		} else {
			println!("Couldn't find any more moves");
			break;
		}
	}
}

// Move made, Resulting Game and total score for branch (set later, initalized at 0)
type NodeData = (Dir, Game);

fn build_decision_tree(game: &Game) -> (NodeId, Tree<NodeData>) {
	let mut tree = Tree::new();
	let top_node = tree.insert(Node::new((Dir::Up, game.clone())), AsRoot).unwrap();
	append_children(&top_node, &mut tree, 0);
	(top_node, tree)
}

fn prune(parent: &NodeId, save: &NodeId, tree: &mut Tree<NodeData>) {
	let sibling_ids: Vec<NodeId> = tree.children_ids(parent).unwrap().cloned().collect();
	for sibling_id in sibling_ids {
		if *save != sibling_id {
			tree.remove_node(sibling_id, DropChildren).unwrap();
		}
	}
}


fn add_layer(node_id: &NodeId, tree: &mut Tree<NodeData>) {
	let children: Vec<NodeId> = tree.children_ids(node_id).unwrap().cloned().collect();

	if children.len() == 0 {
		append_children(node_id, tree, MAX_LEVELS - MOVE_STREAK + 1)
	} else {
		for child in children {
			add_layer(&child, tree)
		}
	}
}

fn opposite(dir: &Dir) -> Dir {
	match *dir {
		Dir::Up => Dir::Down,
		Dir::Right => Dir::Left,
		Dir::Down => Dir::Up,
		Dir::Left => Dir::Right,
	}
}

fn append_children(parent: &NodeId, tree: &mut Tree<NodeData>, level: u32) {
	for d in &[Dir::Up, Dir::Right, Dir::Down, Dir::Left] {
		let mut new_game = tree.get(parent).unwrap().data().1.clone();
		new_game.enter_move(&d, false);
		// Don't let the bot walk into walls or consider paths that walk into walls
		if opposite(d) != tree.get(parent).unwrap().data().0 && new_game.action != Action::WalkedIntoWall {
			let action = new_game.action.clone();
			let new_node = tree.insert(Node::new((d.clone(), new_game)), UnderNode(parent)).unwrap();
			// Keep going if we haven't won yet and haven't filled out max levels
			if action != Action::Won && level < MAX_LEVELS {
				append_children(&new_node, tree, level + 1);
			}
		}
	}
}

fn optimal_step(from: &NodeId, tree: &Tree<NodeData>) -> Option<(Dir, NodeId)> {
	let maybe_max = tree.children_ids(from).unwrap().max_by(|a, b| {
		bottom_score(a, tree).partial_cmp(&bottom_score(b, tree)).unwrap()
	});
	if let Some(max_child) = maybe_max {
		Some((tree.get(max_child).unwrap().data().0.clone(), max_child.clone()))
	} else {
		None
	}
}

fn bottom_score(node_id: &NodeId, tree: &Tree<NodeData>) -> f64 {
	let children: Vec<&NodeId> = tree.children_ids(&node_id).unwrap().collect();
	if children.len() == 0 {
		score_node(node_id, tree)
	} else {
		children.into_iter().fold(-1000., |max, c| {
			let s = bottom_score(&c, tree);
			if s > max {
				s
			} else {
				max
			}
		})
	}
}

fn score_node(node_id: &NodeId, tree: &Tree<NodeData>) -> f64 {
	let g = &tree.get(node_id).unwrap().data().1;
	g.score as f64
	- g.moves as f64 * 0.01
	- g.distance_to_goal() * 0.00001
	+ if g.action == Action::Won { 10. } else { 0. }
}
