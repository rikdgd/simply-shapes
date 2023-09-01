pub mod window_management;
pub mod shapes;
pub mod color;

use window_management::Screen;
use shapes::{Circle, Rectangle, Location};
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
    let test = LogicalSize::new(WIDTH as f64, HEIGHT as f64);

    let mut box_location = Location::new(10, 10);
    let test_shape = Rectangle {
        width: 10,
        height: 10,
        color: Color::new("Green", [0x00, 0x00, 0xff, 0xff]),
    };

    let title = String::from("hello world");
    let test_screen = Screen::new(&title, WIDTH, HEIGHT);

    let game_loop = || {
        box_location.move_x(1);
        box_location.move_y(1);
        test_screen.draw_shape(&test_shape, box_location, frame); // ToDo: look at sample code for example of extracting frames.
    };
}
