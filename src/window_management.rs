use winit::dpi::LogicalSize;
use crate::shapes::Shape;

pub struct Screen {
    pub title: String,
    pub width: u16,
    pub height: u16,
    window: LogicalSize<u16>
}

impl Screen {
    pub fn new(title: String, width: u16, height: u16) -> Self {
        Self {
            title,
            width,
            height,
            window: LogicalSize::new(width, height),
        }
    }
    
    pub fn draw_shape(&self, shape: Shape) {
        
    }
    
    fn fill_color(&self, color: String) {
        
    }
}
