use rand;
use rand::{Rand, Rng};
use std::fmt;
use std::cmp;
use std::collections::{HashMap, HashSet};

static SCORE_3: i32 = 10;
static SCORE_4: i32 = 20;
static SCORE_5: i32 = 30;
static FOLLOWUP_BONUS: i32 = 5;

/// Colors available to be placed on the game board
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Blue,
    Green,
    Orange,
    Purple,
    Red,
    White,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match *self {
            Color::Blue => "B",
            Color::Green => "G",
            Color::Orange => "O",
            Color::Purple => "P",
            Color::Red => "R",
            Color::White => "W",
        };
        write!(f, "{}", c)
    }
}

impl Rand for Color {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let colors = [Color::Blue,
                      Color::Green,
                      Color::Orange,
                      Color::Purple,
                      Color::Red,
                      Color::White];
        rng.choose(&colors).unwrap_or(&Color::Blue).to_owned()
    }
}

/// A grid is just a matrix of colors, the colors are fully owned, this could be optimized by using references
type Grid = Vec<Vec<Color>>;

/// Create a random grid of colors (this will usually include some "invalid" states for a board to display)
pub fn random_grid() -> Grid {
    let mut rng = rand::thread_rng();
    let mut grid = Vec::new();
    for _ in 0..8 {
        let mut row = Vec::new();
        for _ in 0..8 {
            row.push(rng.gen());
        }
        grid.push(row.to_owned())
    }
    grid
}

/// A cell is a pair of indexes to the grid matrix. Assumptions are made throughout the code that the usize's will be >= 0 and <= 7
type Cell = (usize, usize);

/// Return the vertical neighbours (the rows above and below)
fn row_neighbours(cell: &Cell) -> Vec<Cell> {
    let mut cells = Vec::new();
    if cell.0 < 7 {
        cells.push((cell.0 + 1, cell.1));
    }
    if cell.0 > 0 {
        cells.push((cell.0 - 1, cell.1));
    }
    cells
}

/// Return the horizontal neighbours (the cols to the left and right)
fn col_neighbours(cell: &Cell) -> Vec<Cell> {
    let mut cells = Vec::new();
    if cell.1 < 7 {
        cells.push((cell.0, cell.1 + 1));
    }
    if cell.1 > 0 {
        cells.push((cell.0, cell.1 - 1));
    }
    cells
}

/// A move is swapping two pieces at coordinates row1, col1, row2, col2
pub struct Move {
    pub row1: usize,
    pub col1: usize,
    pub row2: usize,
    pub col2: usize,
}

impl Move {
    /// A move is valid if it concerns two neighbouring cells and don't go off the grid
    pub fn is_valid(&self) -> bool {
        if self.row1 < 8 && self.col1 < 8 && self.row2 < 8 && self.col2 < 8 &&
           (self.row1 == self.row2 &&
            cmp::max(self.col1, self.col2) - cmp::min(self.col1, self.col2) == 1 ||
            cmp::max(self.row1, self.row2) - cmp::min(self.row1, self.row2) == 1 &&
            self.col1 == self.col2) {
            return true;
        }
        false
    }
}

/// A game is the collection of stats about the game and its current grid
pub struct Game {
    /// Count of moves so far
    pub moves: i32,
    /// The current score of the game
    pub score: i32,
    /// 8x8 grid of colors
    pub grid: Grid,
}

impl Game {
    /// Create a new game, creates a random board then ensures that it's in a valid state before returning
    pub fn new() -> Game {
        let mut game = Game {
            moves: 0,
            score: 0,
            grid: random_grid(),
        };
        game.clear_board(true);
        game.score = 0;
        game.moves = 0;
        game
    }

    /// A little helper function to pretty-print the board
    pub fn print_board(&self) {
        println!("-----------------");
        println!("{:<8}{:>9}", &self.moves, &self.score);
        println!("-----------------");
        for r in &self.grid {
            for c in r {
                print!("|{}", c);
            }
            println!("|");
        }
        println!("-----------------");
    }

    /// The top level function to make a move on the board, only executed if the move is valid, and "undone" if it doesn't score.
    pub fn make_move(&mut self, mov: &Move) {
        if mov.is_valid() {
            self.moves += 1;
            self.execute_move(mov);
            if self.clear_board(true) == 0 {
                self.execute_move(mov);
            }
        }
    }

    /// Clearing the board means removing any pieces that are in a contiguous area > 3, moving pieces above down one step
    /// and adding a new piece at the top for each removes
    fn clear_board(&mut self, first_loop: bool) -> i32 {
        let xs = self.pieces_to_remove();
        let bonus = if first_loop { 0 } else { FOLLOWUP_BONUS };
        let score = match xs.len() {
            x if x < 3 => return 0,
            3 => SCORE_3 + bonus,
            4 => SCORE_4 + bonus,
            _ => SCORE_5 + bonus,
        };
        self.score += score;
        let mut rng = rand::thread_rng();
        for x in xs {
            for i in 0..x.0 {
                self.execute_move(&Move {
                                      row1: x.0 - 1 - i,
                                      col1: x.1,
                                      row2: x.0 - i,
                                      col2: x.1,
                                  });
            }
            self.grid[0][x.1] = rng.gen();
        }
        score + self.clear_board(false)
    }

    /// Swap the places of two colors on the grid, this could be optimized to remove cloning
    fn execute_move(&mut self, mov: &Move) {
        let moved = self.grid[mov.row1][mov.col1].clone();
        self.grid[mov.row1][mov.col1] = self.grid[mov.row2][mov.col2].clone();
        self.grid[mov.row2][mov.col2] = moved;
    }

    /// Gets all the pieces to remove from the board both row-wise and column-wise
    fn pieces_to_remove(&self) -> Vec<Cell> {
        let mut all_cells = HashSet::new();
        for r in 0..8 {
            for c in 0..8 {
                let mut rows = self.contiguous((r, c), &row_neighbours);
                let mut cols = self.contiguous((r, c), &col_neighbours);
                if rows.len() >= 3 {
                    all_cells = all_cells.union(&mut rows).cloned().collect();
                }
                if cols.len() >= 3 {
                    all_cells = all_cells.union(&mut cols).cloned().collect();
                }
            }
        }
        let mut result = all_cells.drain().collect::<Vec<Cell>>();
        // This sort is important because we need to remove cells from the top down in order
        // to not mutate the pieces that we're working on while we're working on them
        result.sort_by(|a, b| a.0.cmp(&b.0));
        result
    }

    /// Start off the iteration of looking for contiguous cells using `contiguous_loop`
    fn contiguous<F>(&self, cell: Cell, f: &F) -> HashSet<Cell>
        where F: Fn(&Cell) -> Vec<Cell>
    {
        let mut visited: HashMap<Cell, bool> = HashMap::new();
        let mut cells = HashSet::new();
        cells.insert(cell);
        self.contiguous_loop(&mut cells, &mut visited, cell, f);
        cells
    }

    /// Start to accumulate in `contiguous_cells` the neighbours it finds of the same color and iterates to explore deeper
    fn contiguous_loop<F>(&self,
                          contiguous_cells: &mut HashSet<Cell>,
                          visited: &mut HashMap<Cell, bool>,
                          cell: Cell,
                          f: &F)
        where F: Fn(&Cell) -> Vec<Cell>
    {
        if visited.contains_key(&cell) {
            return ();
        } else {
            visited.insert(cell, true);
        }
        for neighbour in f(&cell) {
            if self.grid[cell.0][cell.1] == self.grid[neighbour.0][neighbour.1] &&
               !visited.contains_key(&neighbour) {
                contiguous_cells.insert(neighbour);
                self.contiguous_loop(contiguous_cells, visited, neighbour, f);
            }
        }
    }
}
