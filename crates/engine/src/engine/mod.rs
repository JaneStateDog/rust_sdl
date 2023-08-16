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

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }

    // Printers
    pub fn print_components(&self) {
        for component in self.components.iter() {
            println!("{:?}", component);
        }
    }

    pub fn print_entities(&self) {
        for entity in self.entities.iter() {
            println!("{:?}", entity);
        }
    }

    // Adding things
    pub fn spawn(&mut self, components: Vec<Rc<dyn Component>>) -> &Entity {
        let mut entity_components: Vec<Rc<dyn Component>> = Vec::new();
        for component in &components {
            self.components.push(Rc::clone(component));
            entity_components.push(Rc::clone(self.components.last().unwrap()));
        }

        self.entities.push(Entity::new(entity_components));

        &self.entities.last().unwrap()
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