use crate::game_loop::run_game_loop;

mod event_handling;
mod game_logic;
mod window_management;
mod rendering;
mod game_loop;

pub(crate) const WIDTH: usize = 80;
pub(crate) const HEIGHT: usize = 40;

fn main() {
    let (sdl_context, mut canvas) = window_management::initialize_window().expect("Failed to initialize window");
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    run_game_loop(&mut canvas, &mut event_pump);
}