use crate::shapes::location::Location;

pub trait Brush {
    fn draw(&self, location: Location) -> ();
}