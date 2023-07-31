use std::{
    time,
    ops::{
        Add,
        Sub,
    },
};

// -- VECTOR2 --
#[derive(Clone, Copy)]
pub struct Vector2(
    pub i32, 
    pub i32,
);

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}
// --

// -- ENGINE --
pub struct Engine {
    sdl_context: sdl2::Sdl,

    windows: Vec<Box<Window>>,
    systems: Vec<fn()>,
}

impl Engine {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let windows: Vec<Box<Window>> = Vec::new();
        let systems: Vec<fn()> = Vec::new();

        Self {
            sdl_context,

            windows,
            systems,
        }
    }

    pub fn add_systems(&mut self, systems: &[fn()]) {
        for system in systems {
            self.systems.push(*system);
        }
    }

    fn update(&self) {
        for system in &self.systems{
            system();
        }
    }

    fn render(&self) {
        for window in &self.windows {
            window.render();
        }
    }
}
// --

// -- CLOCK --
pub struct Clock {
    pub target_fps: u32,
    pub target_ups: u32,
    fps: f64,
    ups: f64,

    now: time::Instant,
}

impl Clock {
    pub fn new(target_fps: u32, target_ups: u32) -> Self {
        Self {
            target_fps,
            target_ups,
            fps: target_fps as f64, // This is probably not needed and can just be 0.0
            ups: target_ups as f64,

            now: time::Instant::now(),
        }
    }
}
// --

// -- SURFACE
pub struct Surface {
    size: Vector2,
}

impl Surface {
    pub fn new(size: Vector2) -> Self {
        Self {
            size,
        }
    }
}
// --

// -- WINDOW --
pub struct Window {
    size: Vector2,
    title: String,

    sdl_video_subsystem: sdl2::VideoSubsystem,
    sdl_window: sdl2::video::Window,

    surface: Surface,
}

impl Window {
    pub fn new(self, engine: &mut Engine, title: &str, size: Vector2) -> Self {
        let sdl_video_subsystem = engine.sdl_context.video().unwrap(); // We can get sdl_context from the engine
        // without it being public because this function is defined in the same scope as the engine.
        // Please keep this in mind, Jane, because I feel like you'll almost certainly move stuff around
        // and then wonder why it's not working, lmao.
        let sdl_window = sdl_video_subsystem.window(title, size.0 as u32, size.1 as u32)
            .position_centered()
            .build()
            .unwrap();

        let surface = Surface::new(size);

        engine.windows.push(Box::new(self));

        Self {
            size,
            title: String::from(title),

            sdl_video_subsystem,
            sdl_window,

            surface,
        }
    }

    pub fn render(&self) {
        
    }
}
// --

// -- ENTITY --
pub struct Entity {
    components: Vec<Box<dyn Component>>,
}
// --

// -- COMPONENT --
pub trait Component {
    
}

pub struct TestComponent {

}
impl Component for TestComponent {

}
// --