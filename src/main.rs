pub mod window_management;
pub mod shapes;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;

fn main() {
    println!("Minecraft stack lol: {}", u16::max_value());
    let mut test = LogicalSize::new(WIDTH, HEIGHT);
}