use crate::{
    window::Window,
    entity::Entity,
    component::Component,
};

use std::rc::Rc;

pub struct Engine {
    pub sdl_context: sdl2::Sdl,

    windows: Vec<Window>,
    entities: Vec<Entity>,
    components: Vec<Rc<dyn Component>>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            sdl_context: sdl2::init().unwrap(),

            windows: Vec::new(),
            entities: Vec::new(),
            components: Vec::new(),
        }
    }

    // Getters and setters
    pub fn get_components(&self) -> &Vec<Rc<dyn Component>> {
        &self.components
    }

    // Adding things
    pub fn spawn(&mut self, components: Vec<Rc<dyn Component>>) {
        let mut entity_components: Vec<Rc<dyn Component>> = Vec::new();
        for component in &components {
            self.components.push(Rc::clone(component));
            entity_components.push(Rc::clone(self.components.last().unwrap()));
        }

        self.entities.push(Entity::new(entity_components));
    }

    pub fn add_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    // Run on game tick
    /*fn update(&self) {
        
    }

    fn render(&self) {
        for window in &self.windows {
            window.render();
        }
    }*/
}