use std::collections::HashMap;
use crate::shapes::color::Color;
use crate::shapes::location::Location;



pub trait Shape {
    fn contains_pixel(&self, shape_location: &Location, pixel_location: &Location) -> bool;
    fn get_color(&self) -> &Color;
}


pub struct Circle {
    pub radius: u16,
    pub color: Color,
}

pub struct Rectangle {
    pub width: u16,
    pub height: u16,
    pub color: Color,
}

impl Shape for Rectangle {
    fn contains_pixel(&self, shape_location: &Location, pixel_location: &Location) -> bool {
        let shape_max_x = shape_location.x + self.width;
        let shape_max_y = shape_location.y + self.height;

        let fit_x = pixel_location.x >= shape_location.x && pixel_location.x <= shape_max_x;
        let fit_y =  pixel_location.y >= shape_location.y && pixel_location.y <= shape_max_y;

        fit_x && fit_y
    }

    fn get_color(&self) -> &Color {
        &self.color
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_pixel_rect() {
        let test_color = Color::new("Red", [0xff, 0x00, 0x00, 0xff]);
        let test_rect = Rectangle {
            width: 10,
            height: 10,
            color: test_color,
        };
        let rect_location = Location::new(20, 10); 

        let location_fits = Location::new(22, 15);
        let location_outside = Location::new(100, 100); 
        let location_on_edge = Location::new(30, 15);


        assert!(test_rect.contains_pixel(&rect_location, &location_fits));
        assert!(!test_rect.contains_pixel(&rect_location, &location_outside));
        assert!(test_rect.contains_pixel(&rect_location, &location_on_edge));
    }
}
