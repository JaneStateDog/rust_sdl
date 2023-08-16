use crate::{
    component::*,
    engine::Engine,
};

use std::rc::Rc;

pub struct System<'a, T> {
    pub wanted_components: Vec<ComponentName<'a>>,
    pub function: fn(T),
}

impl<'a, T> System<'a, T> {
    pub fn get_components(&self, engine: &Engine) -> Vec<Rc<dyn Component>> {
        let mut components: Vec<Rc<dyn Component>> = Vec::new();

        for component in engine.get_components() {
            for wanted_component in self.wanted_components.iter() {
                if component.get_name() == *wanted_component {
                    components.push(Rc::clone(component));
                }
            }
        }

        components
    }
}