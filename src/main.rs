use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::game_logic::update_board;
use crate::game_loop::run_game_loop;

mod event_handling;
mod game_logic;
mod window_management;
mod rendering;
mod game_loop;

pub const WIDTH: usize = 40;
pub const HEIGHT: usize = 30;

fn main() {
    let (sdl_context, mut canvas) = window_management::initialize_window().expect("Failed to initialize window");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    run_game_loop(&mut canvas, &mut event_pump);
}