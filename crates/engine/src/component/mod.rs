mod name;

pub use name::ComponentName;

pub trait Component: std::fmt::Debug {
    fn get_name(&self) -> ComponentName;
}