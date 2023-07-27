use std::collections::HashMap;

pub enum Shape {
    Ellipse(EllipseBody),
    Rectangle(RectangleBody),
}


pub struct EllipseBody {
    pub radius: u16,
    pub color: String,
}

pub struct RectangleBody {
    pub width: u16,
    pub height: u16,
    pub color: String,
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

impl Shape {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % 400 as usize) as i16;
            let y = (i / 600 as usize) as i16;

            let inside_the_box = true;

            let rgba = if inside_the_box {
                [0x5e, 0x48, 0xe8, 0xff]
            } else {
                [0x48, 0xb2, 0xe8, 0xff]
            };

            pixel.copy_from_slice(&rgba);
        }
    }
}