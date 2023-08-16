use sdl2::render::WindowCanvas;

pub fn initialize_window() -> Result<(sdl2::Sdl, WindowCanvas), String> {
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