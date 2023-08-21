use std::collections::HashMap;
use crate::color::Color;



pub trait Shape {
    fn within_surface(&self, shape_location: &Location, pixel_location: &Location) -> bool;
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
    fn within_surface(&self, shape_location: &Location, pixel_location: &Location) -> bool {
        let shape_max_x = shape_location.x + self.width;
        let shape_max_y = shape_location.y + self.height;

        let fit_x = pixel_location.x >= shape_location.x && pixel_location.x <= shape_max_x;
        let fit_y =  pixel_location.y >= shape_location.y && pixel_location.y <= shape_max_y;

        fit_x && fit_y
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn within_surface_rect() {
        let test_color = Color::new("Red", [0xff, 0x00, 0x00, 0xff]);
        let test_rect = Rectangle {
            width: 10,
            height: 10,
            color: test_color,
        };
        let rect_location = Location {
            x: 20,
            y: 10,
        };


        let location_fits = Location {
            x: 22,
            y: 15,
        };
        let location_outside = Location {
            x: 100,
            y: 100,
        };
        let location_on_edge = Location {
            x: 30,
            y: 15,
        };


        assert!(test_rect.within_surface(&rect_location, &location_fits));
        assert!(!test_rect.within_surface(&rect_location, &location_outside));
        assert!(test_rect.within_surface(&rect_location, &location_on_edge));
    }
}

