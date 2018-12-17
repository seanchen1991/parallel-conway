use failure::{Error, ResultExt};

use opengl_graphics::*;
use piston::input::*;
use piston::event_loop::*;
use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;

mod life;

// use crate::app::App;
// use crate::cli::{Options};

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Title of the main window
const WINDOW_TITLE: &str = clap::crate_name!();
/// Initial size of the main window
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
    println!("Hello, world!");
}
