use crate::{
    window::Window,
    entity::Entity,
    component::Component,
};

pub struct Engine<'a> {
    pub sdl_context: sdl2::Sdl,

    windows: Vec<Window>,
    entities: Vec<Entity<'a>>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Self {
            sdl_context: sdl2::init().unwrap(),

            windows: Vec::new(),
            entities: Vec::new(),
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
        
    }

    fn render(&self) {
        for window in &self.windows {
            window.render();
        }
    }
}