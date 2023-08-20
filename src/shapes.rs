use std::collections::HashMap;

// TODO: replace enum for trait with common 'get_surface method'.
pub enum Shape {
    Ellipse(EllipseBody),
    Rectangle(RectangleBody),
}

pub trait ShapeTrait {
    fn draw(&self, frame: &mut [u8]);
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
