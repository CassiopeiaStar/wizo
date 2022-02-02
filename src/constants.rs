
use bevy::prelude::*;


pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);


pub const ARENA_WIDTH: f32 = 700.;
pub const ARENA_HEIGHT: f32 = 500.;

pub const LEFT_LANE_POSITION: f32 = -ARENA_WIDTH/4.;
pub const RIGHT_LANE_POSITION: f32 = ARENA_WIDTH/4.;
pub const CENTER_LANE_POSITION: f32 = 0.;
