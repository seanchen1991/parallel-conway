use std::fmt;
use std::iter;
use std::sync::Arc;
use std::num::Wrapping;
use rand::{Rng, thread_rng};

pub struct Board {
    // Vector of booleans representing the state of each cell
    cells: Vec<bool>,
    rows: usize,
    cols: usize,
    // Arc-backed vector of usize's indicating how many live
    // neighbors are needed for a cell to survive or be born
    survive: Arc<Vec<usize>>,
    born: Arc<Vec<usize>>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Board {
        // 3 live neighbors for a dead cell to revive
        let born = vec![3];
        // 2 or 3 live neighbors for an alive cell to survive
        let survive = vec![2, 3];

        // Initialize the Board
        Board::new_with_custom_rules(rows, cols, born, survive) 
    } 

    fn new_with_custom_rules(
        rows: usize,
        cols: usize,
        born: Vec<usize>,
        survive: Vec<usize>,
    ) -> Board {
        // Initialize new cells vector with all false values in parallel
        let new_cells = (0..self.len())
            .into_par_iter()
            .map(|_| {
                false
            }).collect();

        Board {
            cells: new_cells,
            born: Arc::new(born),
            survive: Arc::new(survive),
            rows,
            cols,
        }
    }

    pub fn next_board(&self, new_cells: Vec<bool>) -> Board {
        // Make sure number of cells in `new_cells` matches
        // the number of cells in the current Board
        assert!(new_cells.len() == self.len());

        Board {
            cells: new_cells,
            width: self.width,
            height: self.height,
            born: self.born.clone(),
            survive: self.survive.clone(),
        }
    }
    
    // Generate a new board with randomly-initialized boolean values
    pub fn randomize(&self) -> Board {
        let mut rng = rand::thread_rng();
        let new_cells = (0..self.len())
            .into_par_iter()
            .map(|_| {
                match.rng.gen() {
                    false => false,
                    true => true,
                }
            }).collect();

        self.next_board(new_cells)
    }
    
    /// Get the total number of cells in the Board
    fn len(&self) -> usize {
        self.rows * self.cols
    }

    /// Get the width of the Board as the total number of columns
    pub fn width(&self) -> usize {
        self.cols
    }

    /// Get the height of the Board as the total number of rows
    pub fn height(&self) -> usize {
        self.rows
    }

    /// Get the coordinates of a given cell
    fn get_cell_coordinates(&self, cell: usize) -> (usize, usize) {
        (cell % self.cols, cell / self.cols)
    }

    /// Given the coordinates of a cell, counts the number
    /// of live neighbors of the indicated cell
    fn live_neighbor_count(&self, x: usize, y: usize) -> usize {
        let Wrapping(x_1) = Wrapping(x) - Wrapping(1);
        let Wrapping(y_1) = Wrapping(y) - Wrapping(1);

        let neighbors = [
            self.cell_live(x_1, y_1),
            self.cell_live(x, y_1),
            self.cell_live(x + 1, y + 0),
            self.cell_live(x_1, y + 0),
            self.cell_live(x + 1, y + 0),
            self.cell_live(x_1, y + 1),
            self.cell_live(x, y + 1),
            self.cell_live(x + 1, y + 1),
        ];

        neighbors.iter().filter(|&x| *x).count()
    }

    /// Generate the next board using a parallel iterator
    pub fn next_generation(&self) -> Board {
        let new_cells = (0..self.len())
            .into_par_iter()
            .map(|cell| self.successor_cell(cell))
            .collect();

        self.next_board(new_cells)
    }    

    /// Check a particular neighbor to see whether it is alive or dead
    fn cell_live(&self, x: usize, y: usize) -> bool {
        !(x >= self.cols || y >= self.rows) && self.cells[y * self.cols + x]
    }

    /// Receives the index of a cell
    /// Calculates the cell's coordinates and calls the
    /// `successor` method with those coordinates
    fn successor_cell(&self, cell: usize) -> bool {
        let (x, y) = self.get_cell_coordinates(cell);
        self.successor(x, y)
    }

    /// Returns the next state of the cell at the given coordinates
    fn successor(&self, x: usize, y: usize) -> bool {
        let neighbors = self.live_neighbor_count(x, y);

        if self.cell_live(x, y) {
            self.survive.contains(&neighbors)
        } else {
            self.born.contains(&neighbors)
        }
    }
}
