use failure::{Error, ResultExt};

use opengl_graphics::*;
use piston::input::*;
use piston::event_loop::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;

mod life;

// use crate::app::App;
// use crate::cli::{Options};

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Title of the main window
const WINDOW_TITLE: &str = "Conway's Game of Life";
/// Initial size of the main window
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
    let let Err(error) = run() {
        eprintln!("Error: {}", error);

        for cause in error.iter_causes() {
            eprintln!("Caused by: {}", cause);
        }

        eprintln!("{}", error.backtrace());
        
        use std::process;
        process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    
}
