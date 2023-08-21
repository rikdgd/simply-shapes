use std::collections::HashMap;
use crate::color::Color;


pub trait Shape {
    fn within_surface(&self, location: Location) -> bool;
}

pub struct Location {
    pub x: u16,
    pub y: u16,
}

pub struct Ellipse {
    pub radius: u16,
    pub color: Color,
}

pub struct Rectangle {
    pub width: u16,
    pub height: u16,
    pub color: Color,
}

impl Shape for Rectangle {
    fn within_surface(&self, location: Location) -> bool {
        // ToDo decide how/where the location of the shape should be stored.
        false
    }
}
