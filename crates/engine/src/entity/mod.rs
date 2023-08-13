use crate::component::Component;

pub struct Entity<'a> {
    pub components: Vec<&'a dyn Component>,
}

impl<'a> Entity<'a> {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
}