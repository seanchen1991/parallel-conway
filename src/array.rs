//! The Array struct

use graphics::color;
use graphics::types::Color;

use crate::state::SharedState;

/// The Array struct acts as a convenience wrapper around
/// the SharedState struct. It deals with ensuring that 
/// SharedState is accessed concurrently in a safe manner.
/// All methods in this struct lock the SharedState for as
/// short a time as possible so that the rendering thread
/// can get the lock to the SharedState as often as it needs to.
#[derive(Debug)]
pub struct Array(SharedState);

impl Array {
    /// Creates a new Array from a copy of SharedState
    pub fn new(state: SharedState) -> Array {
        Array(state)
    }

    /// Puts the current thread to sleep for the specified amount
    /// of time and blocks it if the animation is paused.
    pub fn wait(&self, ms: u64) {
        use std::thread;
        use std::time::Duration;

        // Don't keep SharedState locked while it is sleeping
        thread::sleep(Duration::from_micros({
            let state = self.0.get();
            (ms as f64 * 1000.0 / state.speed) as u64
        }));

        let paused = {
            let state = self.0.get();
            state.paused
        };

        if paused {
            thread::park();
        }
    }

    /// Returns the length of the underlying Board in SharedState
    pub fn len(&self) -> usize {
        let state = self.0.get();
        state.board.len()
    }
}