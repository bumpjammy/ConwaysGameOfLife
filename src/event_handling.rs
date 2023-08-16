use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use crate::{game_logic, HEIGHT, WIDTH};

pub(crate) fn handle_event(event: Event, board: &mut Vec<bool>, playing: &mut bool, speed: &mut isize) {
    match event {
        Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
            game_logic::update_board(&mut *board, WIDTH, HEIGHT);
        },
        Event::KeyDown { keycode: Some(Keycode::P), .. } => {
            *playing = !*playing;
        },
        Event::KeyDown { keycode: Some(Keycode::R), ..} => {
            *board = vec![false; WIDTH * HEIGHT];
        },
        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
            if *speed < 60 {
                *speed += 10;
            }
        },
        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
            if *speed > 10 {
                *speed -= 10;
            }
        },
        Event::MouseButtonDown { x, y, .. } => {
            let square_x: usize = (x / 20) as usize;
            let square_y: usize = (y / 20) as usize;
            let clicked_index = square_x * HEIGHT + square_y;
            board[clicked_index] ^= true;
        }
        _ => {}
    }
}