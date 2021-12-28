use macroquad::prelude::Vec2;

pub(crate) enum MiniMapPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub(crate) struct MiniMap {
    pos: Vec2,
    width: f32,
    height: f32,
    size: f32,
}

impl MiniMap {
    pub fn new(pos: MiniMapPosition, size: f32, screen_width: f32, screen_height: f32) -> Self {
        let width = screen_width * size;
        let height = screen_height * size;

        let pos = match pos {
            MiniMapPosition::TopLeft => Vec2::new(0_f32, 0_f32),
            MiniMapPosition::TopRight => Vec2::new(screen_width - width, 0_f32),
            MiniMapPosition::BottomLeft => Vec2::new(0_f32, screen_height - height),
            MiniMapPosition::BottomRight => Vec2::new(screen_width - width, screen_height - height),
        };

        MiniMap {
            pos,
            size,
            width,
            height,
        }
    }
}
