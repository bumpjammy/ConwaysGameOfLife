use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureQuery, WindowCanvas};

pub fn draw_board(canvas: &mut WindowCanvas, board: &[bool], width: usize, height: usize) {
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

    render_text(&mut *canvas, "Conway's Game Of Life", 10, 10, 30);
    canvas.present();
}

fn render_text(canvas: &mut WindowCanvas, text: &str, x: i32, y: i32, size : u16) {
    let ttf_context = sdl2::ttf::init().unwrap();
    let font_path = std::path::Path::new("assets/Amatic-Bold.ttf"); //If you get an error here, you need to download the font and put it in the assets folder
    let font = ttf_context.load_font(font_path, size).unwrap();
    let surface = font.render(text).blended(Color::RGBA(255, 255, 255, 150)).unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.create_texture_from_surface(&surface).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let target = Rect::new(x, y, width, height);
    canvas.copy(&texture, None, Some(target)).unwrap();
}