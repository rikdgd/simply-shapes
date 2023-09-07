use crate::color::Color;
use crate::shapes::Shape;
use crate::shapes::Location;
use crate::shapes::Rectangle;
use crate::window_management::Screen;
use crate::brushes::brush::Brush;

pub struct ShapeBrush<'a> {
    screen: &'a Screen<'a>,
    shape: &'a dyn Shape,
}

impl<'a> ShapeBrush<'a> {
    pub fn new(screen: &'a Screen, shape: &'a impl Shape) -> Self {
        Self {
            screen,
            shape,
        }
    }
}

impl Brush for ShapeBrush<'_> {
    fn draw(&self, location: Location) {
        println!("Drawing shape");
    }
}



// pub fn modify_screen<'a>(screen: Screen<'a>, shape: &'a impl Shape, shape_location: &'a Location) -> Screen<'a> {
//     let mut pixels = {
//         let window_size = screen.window.inner_size();
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &screen.window);
//         Pixels::new(screen.width as u32, screen.height as u32, surface_texture).unwrap()
//     };

//     let frame = pixels.frame_mut();
    
//     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
//         let x = (i % screen.width as usize) as u16;
//         let y = (i / screen.height as usize) as u16;
//         let pixel_location = Location::new(x, y);

//         let inside_shape = shape.contains_pixel(&shape_location, &pixel_location);
        
//         let rgba = if inside_shape {
//             shape.get_color().rgba
//         } else {
//             [0xff, 0xff, 0xff, 0xff]    // Black
//         };
        
//         pixel.copy_from_slice(&rgba);
//     }

//     screen
// }

