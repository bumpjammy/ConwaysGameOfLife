use std::time::Duration;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use crate::{event_handling, HEIGHT, rendering, WIDTH};
use crate::game_logic::update_board;

enum Speed {
    Fast = (1_000_000_000u32 / 60) as isize,
    Medium = (1_000_000_000u32 / 30) as isize,
    Slow = (1_000_000_000u32 / 10) as isize,
}

pub fn run_game_loop(canvas: &mut WindowCanvas, event_pump: &mut EventPump) {

    let mut speed = 30;
    let mut board = vec![false; WIDTH * HEIGHT];
    let mut playing = false;

    'running: loop {

        rendering::draw_board(&mut *canvas, &board, WIDTH, HEIGHT);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {
                    event_handling::handle_event(event, &mut board, &mut playing, &mut speed);
                }
            }
        }

        if playing {
            update_board(&mut board, WIDTH, HEIGHT);
        }

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / speed as u32));
    }
}