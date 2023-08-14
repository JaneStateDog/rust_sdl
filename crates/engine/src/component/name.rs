use std::fmt;

#[derive(PartialEq, Debug)]
pub struct ComponentName(pub String);

impl fmt::Display for ComponentName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ComponentName {
    pub fn new(name: &str) -> Self {
        Self(name.into())
    }
}