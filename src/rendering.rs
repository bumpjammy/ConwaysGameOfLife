use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};
use sdl2::ttf::{Font, Sdl2TtfContext};
use crate::HEIGHT;

pub(crate) fn draw_board(canvas: &mut WindowCanvas, board: &[bool], width: usize, height: usize) {
    canvas.set_draw_color(Color::RGB(70, 70, 70));
    canvas.clear();

    let mut square = (0, 0);
    while square.1 < width {
        while square.0 < height {
            canvas.set_draw_color(Color::RGB(49, 49, 49));
            if board[square.1 * height + square.0] {
                canvas.set_draw_color(Color::RGB(220, 220, 0));
            }
            canvas.fill_rect(Rect::new((square.1 * 20 + 2) as i32, (square.0 * 20 + 2) as i32, 19, 19))
                .expect("Failed to create rectangle");
            square.0 += 1;
        }
        square.0 = 0;
        square.1 += 1;
    }
}

pub(crate) fn load_font<'a>(font_location: &'a str, ttf_context: &'a Sdl2TtfContext, size: u16) -> Font<'a, 'a> {
    let font_path = std::path::Path::new(font_location); //If you get an error here, you need to download the font and put it in the assets folder
    ttf_context.load_font(font_path, size).expect("Failed to load font")
}

pub(crate) fn render_instructions(canvas: &mut WindowCanvas, small_font: &Font, large_font: &Font, playing: bool, speed: isize, generation: usize) {
    render_text(&mut *canvas, "Press P to play/pause", 10, 5, &small_font);
    render_text(&mut *canvas, "Press R to reset", 10, 35, &small_font);
    render_text(&mut *canvas, "Press Space to step", 10, 65, &small_font);
    render_text(&mut *canvas, "Press Up/Down to change speed", 10, 95, &small_font);
    if playing {
        render_text(&mut *canvas, "Playing", 10, ((HEIGHT - 7) * 20) as i32, &large_font);
    } else {
        render_text(&mut *canvas, "Paused", 10, ((HEIGHT - 7) * 20) as i32, &large_font);
    }
    render_text(&mut *canvas, &format!("Speed - {}", speed), 10, ((HEIGHT - 5) * 20) as i32, &large_font);
    render_text(&mut *canvas, &format!("Generation - {}", generation), 10, ((HEIGHT - 3) * 20) as i32, &large_font);
}

pub(crate) fn render_text(canvas: &mut WindowCanvas, text: &str, x: i32, y: i32, font: &Font) {
    let surface = font.render(text).blended(Color::RGBA(255, 255, 255, 100)).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(x, y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
}