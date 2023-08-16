use crate::useful::Vec2;

#[allow(dead_code)]
pub struct Surface {
    size: Vec2,
}

impl Surface {
    pub fn new(size: Vec2) -> Self {
        Self {
            size,
        }
    }
}