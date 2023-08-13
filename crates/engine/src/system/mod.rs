use crate::component::*;

pub struct System<'a, T> {
    pub wanted_components: Vec<ComponentName<'a>>,
    pub function: fn(T),
}

impl<'a, T> System<'a, T> {
    pub fn get_components(&self) -> Vec<&dyn Component> {
        // TODO: Get a list of the components that follow our wanted component types

        Vec::new()
    }
}