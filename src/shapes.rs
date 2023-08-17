use std::collections::HashMap;

// TODO: replace enum for trait with common 'get_surface method'.
pub enum Shape {
    Ellipse(EllipseBody),
    Rectangle(RectangleBody),
}

pub struct Color {
    name: String,
    rgba: [i32; 4],
}
impl Color {
    pub fn new(name: &str, rgba: [i32; 4]) -> Self {
        Self {
            name: String::from(name),
            rgba,
        }
    }
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

// Returns some of the basic colors.
pub fn color_presets() -> Vec<Color> {
    let mut color_presets = Vec::new();
    
    color_presets.push(Color::new("Red", [0xff, 0x00, 0x00, 0xff]));
    color_presets.push(Color {name: "Green", rgba: [0x00, 0x00, 0xff, 0xff]});
    color_presets.push(Color {name: "Blue", rgba: [0x00, 0xff, 0x00, 0xff]});
    
    color_presets.push(Color {name: "Yellow", rgba: [0xff, 0xff, 0x00, 0xff]});
    color_presets.push(Color {name: "Cyan", rgba: [0x00, 0xff, 0xff, 0xff]});
    color_presets.push(Color {name: "Pink", rgba: [0xff, 0x00, 0xff, 0xff]});
    
    color_presets    
}

// ToDo: move this method to screen
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
