pub struct Location {
    pub x: u16,
    pub y: u16,
}

impl Location {
    pub fn new(x: u16, y: u16) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn move_x(&mut self, x: u16) {
        self.x += x;
    }

    pub fn move_y(&mut self, y: u16) {
        self.y += y;
    }
}
