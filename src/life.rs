use time;

use std::thread;
use std::sync::Arc;
use std::iter::repeat;
use std::num::Wrapping;

use rand::{thread_rng, Rng};
use rand::distributions::Standard;

use rayon::prelude::*;

#[derive(Deserialize)]
pub struct Args {
    cmd_bench: bool,
    cmd_play: bool,
    flag_size: usize,
    flag_gens: usize,
    flap_fps: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Board {
    board: Vec<bool>,
    survive: Arc<Vec<usize>>,
    born: Arc<Vec<usize>>,
    rows: usize,
    cols: usize,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        let born = vec![3];
        let survive = vec![2, 3];

        Board::new_with_custom_rules(rows, cols, born, survive)
    }

    fn new_with_custom_rules(
        rows: usize,
        cols: usize,
        born: Vec<usize>,
        survive: Vec<usize>,
    ) -> Board {
        let new_board = repeat(false).take(rows * cols).collect();

        Board {
            board: new_board,
            born: Arc::new(born),
            survive: Arc::new(survive),
            rows,
            cols,
        }
    }

    fn len(&self) -> usize {
        self.rows * self.cols
    }

    fn next_board(&self, new_board: Vec<bool>) -> Board {
        assert!(new_board.len() == self.len());

        Board {
            board: new_board,
            born: self.born.clone(),
            survive: self.survive.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    pub fn random(&self) -> Board {
        let new_board = thread_rng()
            .sample_iter(&Standard)
            .take(self.len())
            .collect();

        self.next_board(new_board)
    }

    pub fn next_generation(&self) -> Board {
        let new_board = (0..self.len())
            .into_par_iter()
            .map(|cell| self.successor_cell(cell))
            .collect();

        self.next_board(new_board)
    }

    fn cell_live(&self, x: usize, y: usize) -> bool {
        !(x >= self.cols || y >= self.rows) && self.board[y * self.cols + x]
    }

    fn living_neighbors(&self, x: usize, y: usize) -> usize {
        let Wrapping(x_1) = Wrapping(x) - Wrapping(1);
        let Wrapping(y_1) = Wrapping(y) - Wrapping(1);
        let neighbors = [
            self.cell_live(x_1, y_1),
            self.cell_live(x, y_1),
            self.cell_live(x + 1, y_1),
            self.cell_live(x_1, y + 0),
            self.cell_live(x + 1, y + 0),
            self.cell_live(x_1, y + 1),
            self.cell_live(x, y + 1),
            self.cell_live(x + 1, y + 1),
        ];
        neighbors.iter().filter(|&x| *x).count()
    }

    fn successor_cell(&self, cell: usize) -> bool {
        self.successor(cell % self.cols, cell / self.cols)
    }

    fn successor(&self, x: usize, y: usize) -> bool {
        let neighbors = self.living_neighbors(x, y);
        if self.cell_live(x, y) {
            self.survive.contains(&neighbors)
        } else {
            self.born.contains(&neighbors)
        }
    }
}

fn generations(board: Board, gens: usize) {
    let mut brd = board;
    for _ in 0..gens {
        brd = brd.next_generation();
    }
}

fn delay(last_start: u64, min_interval_ns: u64) -> u64 {
    let mut current_time = time::precise_time_ns();
    let elapsed = current_time - last_start;
    if elapsed < min_interval_ns {
        let delay = min_interval_ns - elapsed;
        thread::sleep(::std::time::Duration::from_nanos(delay));
        current_time += delay;
    }
    current_time
}

fn generations_limited(board: Board, gens: usize, min_interval_ns: u64) {
    let mut brd = board;
    let mut time = time::precise_time_ns();
    for _ in 0..gens {
        brd = brd.next_generation();
        time = delay(time, min_interval_ns);
    }
}

fn measure(f: fn(Board, usize) -> (), args: &Args) -> u64 {
    let (n, gens) = (args.flag_size, args.flag_gens);
    let brd = Board::new(n, n).random();
    let start = time::precise_time_ns();

    f(brd, gens);

    time::precise_time_ns - start
}


