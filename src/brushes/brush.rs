use crate::shapes::Location;

pub trait Brush {
    fn draw(&self, location: Location) -> ();
}