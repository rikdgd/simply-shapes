use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;
use crate::shapes::{Shape, Location};
use crate::color::Color;
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
    
    pub fn draw_shape<T>(&self, shape: &T, shape_location: Location, frame: &mut [u8])
        where 
            T: Shape
        {
        let mut pixels = {
            let window_size = self.window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &self.window);
            Pixels::new(self.width as u32, self.height as u32, surface_texture).unwrap()
        };
        
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (i % self.width as usize) as u16;
            let y = (i / self.height as usize) as u16;
            let pixel_location = Location::new(x, y);

            let inside_shape = shape.contains_pixel(&shape_location, &pixel_location);
            
            let rgba = if inside_shape {
                shape.get_color().rgba
            } else {
                [0xff, 0xff, 0xff, 0xff]    // Black
            };
            
            pixel.copy_from_slice(&rgba);
        }
    }
    
    fn fill_color(&self, color: String) {
        
    }
}
