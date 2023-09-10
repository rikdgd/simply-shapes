use simply_shapes::screen_management::unified_screen::UnifiedScreen;
use simply_shapes::shapes::shapes::{Circle, Rectangle, Location};
use simply_shapes::shapes::color::Color;
use simply_shapes::brushes::shape_brush;


const WIDTH: u16 = 800;
const HEIGHT: u16 = 600;


fn main() {
    let title = String::from("hello world");


    let mut box_location = Location::new(10, 10);
    let test_shape = Rectangle {
        width: 10,
        height: 10,
        color: Color::new("Green", [0x00, 0x00, 0xff, 0xff]),
    };

    let game_loop = |screen: Screen| {
        box_location.move_x(1);
        box_location.move_y(1);

        modify_screen(screen, &test_shape, &box_location);
        // screen.draw_shape(&test_shape, box_location, frame); // ToDo: look at sample code for example of extracting frames.
    };

    let test_screen = Screen::new(&title, WIDTH, HEIGHT, &game_loop);


}
