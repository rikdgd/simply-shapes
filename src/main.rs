pub mod window_management;
pub mod shapes;
pub mod color;

use window_management::Screen;
use shapes::{Ellipse, Rectangle};
use pixels::{Error, Pixels, SurfaceTexture};
use color::Color;

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;


fn main() {
    println!("Minecraft stack lol: {}", u16::max_value());
    
    let test = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

    let title = String::from("hello world");
    let test_screen = Screen::new(&title, WIDTH, HEIGHT);
}
