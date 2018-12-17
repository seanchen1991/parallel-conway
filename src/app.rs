use graphics::color::{BLACK, WHITE};
use graphics::types::Color;

use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::*;
use std::thread;

pub const ALIVE_COLOR: Color = BLACK;
pub const DEAD_COLOR: Color = WHITE;