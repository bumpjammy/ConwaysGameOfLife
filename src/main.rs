use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

fn main() {
    open_window().expect("Failed to open window");
}

fn open_window() -> Result<(), String> {
    let (sdl_context, mut canvas) = initialize_window()?;
    let mut event_pump = sdl_context.event_pump()?;

    const WIDTH: usize = 40;
    const HEIGHT: usize = 30;
    let mut board = vec![false; WIDTH * HEIGHT];

    'running: loop {

        draw_board(&mut canvas, &board, WIDTH, HEIGHT);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    update_board(&mut board, WIDTH, HEIGHT);
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

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn initialize_window() -> Result<(sdl2::Sdl, WindowCanvas), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Conway's Game Of Life", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to initialize");

    let canvas = window.into_canvas().build()
        .expect("Failed to create canvas");

    Ok((sdl_context, canvas))
}

fn draw_board(canvas: &mut WindowCanvas, board: &[bool], width: usize, height: usize) {
    canvas.set_draw_color(Color::RGB(130, 130, 130));
    canvas.clear();

    let mut square = (0, 0);
    while square.1 < width {
        while square.0 < height {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            if board[square.1 * height + square.0] {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.fill_rect(Rect::new((square.1 * 20 + 2) as i32, (square.0 * 20 + 2) as i32, 19, 19))
                .expect("Failed to create rectangle");
            square.0 += 1;
        }
        square.0 = 0;
        square.1 += 1;
    }
}

fn update_board(board: &mut Vec<bool>, width: usize, height: usize) {
    let mut new_board = vec![false; width * height];
    for (index, cell) in board.iter().enumerate() {
        let neighbors = get_neighbours(index as i32, height as i32, width as i32);
        let mut alive_neighbors = 0;
        for neighbor in neighbors {
            if board[neighbor as usize] {
                alive_neighbors += 1;
            }
        }
        if *cell {
            if alive_neighbors == 2 || alive_neighbors == 3 {
                new_board[index] = true;
            }
        } else {
            if alive_neighbors == 3 {
                new_board[index] = true;
            }
        }
    }
    *board = new_board;
}

fn get_neighbours(index: i32, height: i32, width: i32) -> Vec<i32> {
    let mut neighbors = Vec::new();

    let x = index / height;
    let y = index % height;

    let relative_neighbors = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),          (1,  0),
        (-1,  1), (0,  1), (1,  1)
    ];

    for neighbor in relative_neighbors {
        let mut neighbor_x = x + neighbor.0;
        let mut neighbor_y = y + neighbor.1;
        // Deal with wrapping
        if neighbor_x < 0 {
            neighbor_x = width - 1;
        } else if neighbor_x >= width {
            neighbor_x = 0;
        }
        if neighbor_y < 0 {
            neighbor_y = height - 1;
        } else if neighbor_y >= height {
            neighbor_y = 0;
        }

        neighbors.push(neighbor_y + neighbor_x * height);
    }

    neighbors
}