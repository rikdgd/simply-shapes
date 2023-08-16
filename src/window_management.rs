use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;
use crate::shapes::Shape;
use pixels::{Pixels, SurfaceTexture};

pub struct Screen {
    pub title: String,
    pub width: u16,
    pub height: u16,
    window: Window,
    pub event_loop: EventLoop<()>,
}

impl Screen {
    pub fn new(title: &str, width: u16, height: u16) -> Self {
        let logical_size = LogicalSize::new(width, height);
        let event_loop = EventLoop::new();
        
        Self {
            title: String::from(title),
            width,
            height,
            window: WindowBuilder::new()
                .with_title(title.clone())
                .with_inner_size(logical_size)
                .with_min_inner_size(logical_size)
                .build(&event_loop)
                .unwrap(),
            event_loop
        }
    }
    
    pub fn draw_shape(&self, shape: Shape, frame: &mut [u8]) {
        let mut pixels = {
            let window_size = self.window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &self.window);
            Pixels::new(self.width as u32, self.height as u32, surface_texture).unwrap()
        };
        
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as i16;
            let y = (i / self.height as usize) as i16;
            
            let inside_shape = true; // ToDo: Calculate if the pixel should be colored.
            
            let rgba = if inside_shape {
                //shape.rgba
                [0x5e, 0x48, 0xe8, 0xff] // placeholder
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };
        }
    }
    
    pub fn draw_shapes(&self, shapes: Vec<Shape>, frame: &mut [u8]) {
        for shape in shapes {
            self.draw_shape(shape, frame);
        }
    }
    
    fn fill_color(&self, color: String) {
        
    }
}
