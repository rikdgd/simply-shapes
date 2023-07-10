use std::collections::HashMap;

pub enum Shape {
    Ellipse(EllipseBody),
    Rectangle(RectangleBody),
}


pub struct EllipseBody {
    radius: u16,
    color: String,
}

pub struct RectangleBody {
    width: u16,
    height: u16,
    color: String,
}

// Returns a hashmap with some color presets.
pub fn colors() -> HashMap<String,[i32; 4]> {
    let mut color_map = HashMap::new();
    
    color_map.insert(String::from("Red"), [0xff, 0x00, 0x00, 0xff]);
    color_map.insert(String::from("Green'"), [0x00, 0x00, 0xff, 0xff]);
    color_map.insert(String::from("Blue'"), [0x00, 0xff, 0x00, 0xff]);
    
    color_map.insert(String::from("Yellow'"), [0xff, 0xff, 0x00, 0xff]);
    color_map.insert(String::from("Cyan'"), [0x00, 0xff, 0xff, 0xff]);
    color_map.insert(String::from("Pink'"), [0xff, 0x00, 0xff, 0xff]);
    
    color_map
}