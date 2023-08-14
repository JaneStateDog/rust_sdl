use crate::component::Component;

use std::rc::Rc;

pub struct Entity {
    pub components: Vec<Rc<dyn Component>>,
}

impl Entity {
    pub fn new(components: Vec<Rc<dyn Component>>) -> Self {
        Self {
            components,
        }
    }
}