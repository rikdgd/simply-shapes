pub struct Color {
    pub name: String,
    pub rgba: [i32; 4],
}
impl Color {
    pub fn new(name: &str, rgba: [i32; 4]) -> Self {
        Self {
            name: String::from(name),
            rgba,
        }
    }
}

// Returns some of the basic colors.
pub fn color_presets() -> Vec<Color> {
    let mut color_presets = Vec::new();
    
    color_presets.push(Color::new("Red", [0xff, 0x00, 0x00, 0xff]));
    color_presets.push(Color::new("Green", [0x00, 0x00, 0xff, 0xff]));
    color_presets.push(Color::new("Blue", [0x00, 0xff, 0x00, 0xff]));

    color_presets.push(Color::new("Yellow", [0xff, 0xff, 0x00, 0xff]));
    color_presets.push(Color::new("Cyan", [0x00, 0xff, 0xff, 0xff]));
    color_presets.push(Color::new("Pink", [0xff, 0x00, 0xff, 0xff]));
    
    color_presets    
}
