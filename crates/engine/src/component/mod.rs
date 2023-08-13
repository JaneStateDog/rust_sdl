mod name;

pub use name::ComponentName;

pub trait Component {
    fn get_name(&self) -> ComponentName;
}