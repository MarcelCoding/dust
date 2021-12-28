use macroquad::math::Vec2;

pub(crate) struct Screen {
    width: f32,
    height: f32,
    middle: Vec2,
}

impl Screen {
    pub(crate) fn new() -> Self {
        Screen {
            width: 0_f32,
            height: 0_f32,
            middle: Vec2::new(0_f32, 0_f32),
        }
    }

    pub(crate) fn sync(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.middle.x = width / 2_f32;
        self.middle.y = height / 2_f32;
    }

    pub(crate) fn width(&self) -> f32 {
        self.width
    }

    pub(crate) fn height(&self) -> f32 {
        self.height
    }

    pub(crate) fn middle(&self) -> Vec2 {
        self.middle
    }

    pub(crate) fn x_middle(&self) -> f32 {
        self.middle.x
    }

    pub(crate) fn y_middle(&self) -> f32 {
        self.middle.y
    }
}
