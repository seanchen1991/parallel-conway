//! Structs and type definitions that describe the app state.

use std::sync::{Arc, Mutex, MutexGuard};

use graphics::types::Color;

use crate::life::Board;

/// Contains the state of whole application
#[derive(Debug)]
pub struct State {
    /// Current time in seconds. Updates if the animation is not paused.
    pub time: f64,
    /// Speed factor (e.g. 1.0 - normal, 2.0 - 2x faster, 0.5 - 2x slower, etc.)
    /// Affects the speed of the animation 
    pub speed: f64,
    /// Is the animation paused?
    pub paused: bool,
    /// The current Board
    pub board: Board,
    /// Colored overlays for each cell value
    /// The length of this vector matches the length
    /// of the Board, so every color in this vector corresponds
    /// with a cell value in the Board
    pub colors: Vec<Color>,
}

/// A wrapper around [`State`] that can be shared between threads.
#[derive(Clone, Debug)]
pub struct SharedState(Arc<Mutex<State>>);

impl SharedState {
    pub fn new(state: State) -> SharedState {
        SharedState(Arc::new(Mutex::new(state)))
    }

    pub fn get(&self) -> MutexGuard<'_, State> {
        self.0.lock().unwrap()
    }
}