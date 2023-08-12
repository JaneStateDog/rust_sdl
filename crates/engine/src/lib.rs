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
pub struct Engine<'a> {
    sdl_context: sdl2::Sdl,

    windows: Vec<Window>,
    systems: Vec<fn()>,
    entities: Vec<Entity<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            sdl_context: sdl2::init().unwrap(),

            windows: Vec::new(),
            systems: Vec::new(),
            entities: Vec::new(),
        }
    }

    pub fn add_systems(&mut self, systems: &[fn()]) {
        for system in systems {
            self.systems.push(*system);
        }
    }

    pub fn spawn(&mut self, components: Vec<&'a dyn Component>) {
        self.entities.push(Entity {
            components
        });
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }


    // --- RUN ON GAME TICK ---
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
    pub fn new(engine: &mut Engine, title: &str, size: Vector2) -> Self {
        let sdl_video_subsystem = engine.sdl_context.video().unwrap(); // We can get sdl_context from the engine
        // without it being public because this function is defined in the same scope as the engine.
        // Please keep this in mind, Jane, because I feel like you'll almost certainly move stuff around
        // and then wonder why it's not working, lmao.

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
// --

// -- ENTITY --
pub struct Entity<'a> {
    components: Vec<&'a dyn Component>,
}

impl<'a> Entity<'a> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}
// --

// -- COMPONENT --
pub trait Component {
    fn get_name(&self) -> String;
}
// --

// -- SYSTEM --
pub struct System {
    wanted_components: Vec<String>,
    function: dyn Fn(),
}