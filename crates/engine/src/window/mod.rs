use crate::{
    useful::Vector2,
    surface::Surface,
    engine::Engine,
};

pub struct Window {
    size: Vector2,
    title: String,

    sdl_video_subsystem: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,

    surface: Surface,
}

impl Window {
    pub fn new(engine: &mut Engine, title: &str, size: Vector2) -> Self {
        let sdl_video_subsystem = engine.sdl_context.video().unwrap();
        let sdl_window = sdl_video_subsystem.window(title, size.0 as u32, size.1 as u32)
            .position_centered()
            .build()
            .unwrap();

        Self {
            size,
            title: String::from(title),

            sdl_video_subsystem,
            sdl_window,

            surface: Surface::new(size),
        }
    }

    pub fn render(&self) {
        
    }
}