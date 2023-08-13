use crate::useful::Vector2;

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