use crate::component::*;

use std::rc::Rc;

#[derive(Debug)]
pub struct Entity {
    pub components: Vec<Rc<dyn Component>>,
}

impl Entity {
    pub fn new(components: Vec<Rc<dyn Component>>) -> Self {
        Self {
            components,
        }
    }

    // Getters and setters
    pub fn get_components_of_name(&self, name: ComponentName) -> Vec<Rc<dyn Component>> {
        let mut output = Vec::new();
        for component in &self.components {
            if component.get_name() == name {
                output.push(Rc::clone(component));
            }
        }

        output
    }
}