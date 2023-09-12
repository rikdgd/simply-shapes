use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{WindowBuilder, Window};
use winit_input_helper::WinitInputHelper;
use pixels::{Pixels, SurfaceTexture};
use crate::screen_management::screen::Screen;
use crate::shapes::color::Color;

pub struct UnifiedScreen<'a>  {
    pub title: String,
    pub width: u16,
    pub height: u16,
    window: Window,
    pixels: Pixels,
    pub event_loop: EventLoop<()>,
    pub main_loop: &'a mut dyn FnMut(&mut Pixels) -> (),
}


impl<'a> Screen for UnifiedScreen<'a> {
    fn start(&mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            // ToDo: check if redraw is requested.
            //self.main_loop(self.pixels);
            // ToDo: request redraw
        })
    }

    fn stop(&mut self) {
        println!("this screens title is: {}", self.title);
    }

    fn fill(&mut self, color: &Color) {
        println!("filling with the given color...");
    }
}


impl<'a> UnifiedScreen<'a> {
    pub fn new(title: &str, width: u16, height: u16, main_loop: &'a mut dyn FnMut(&mut Pixels) -> ()) -> Self {
        let logical_size = LogicalSize::new(width, height);
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
        .with_title(title.clone())
        .with_inner_size(logical_size)
        .with_min_inner_size(logical_size)
        .build(&event_loop)
        .unwrap();

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(width as u32, height as u32, surface_texture).unwrap()
        };

        
        Self {
            title: String::from(title),
            width,
            height,
            window,
            pixels,
            event_loop,
            main_loop,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn set_main_loop(&mut self, main_loop: &'a mut dyn FnMut(&mut Pixels) -> ()) {
        self.main_loop = main_loop;
    }
    
    // pub fn draw_shape<T>(&self, shape: &T, shape_location: Location, frame: &mut [u8])
    //     where 
    //         T: Shape
    //     {
        // let mut pixels = {
        //     let window_size = self.window.inner_size();
        //     let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &self.window);
        //     Pixels::new(self.width as u32, self.height as u32, surface_texture).unwrap()
        // };
        
    //     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
    //         let x = (i % self.width as usize) as u16;
    //         let y = (i / self.height as usize) as u16;
    //         let pixel_location = Location::new(x, y);

    //         let inside_shape = shape.contains_pixel(&shape_location, &pixel_location);
            
    //         let rgba = if inside_shape {
    //             shape.get_color().rgba
    //         } else {
    //             [0xff, 0xff, 0xff, 0xff]    // Black
    //         };
            
    //         pixel.copy_from_slice(&rgba);
    //     }
    // }
}


// ToDo: macros need testing for sure, since there is no type checking.
macro_rules! start_loop {
    ($screen:expr) => {
        $screen.event_loop.run(move |event, _, control_flow| {
            // run the custom event loop.
        });
    }
}

pub(crate) use start_loop;
