use std::f32::consts::PI;

use macroquad::prelude::{IVec2, Vec2};

use crate::Screen;

const NINETY_DEGREES_IN_RAD: f32 = PI / 2_f32;

#[derive(PartialEq)]
pub(crate) enum Side {
    NS,
    EW,
}

pub(crate) struct Camera {
    pos: Vec2,
    direction: Vec2,
    projection_distance: f32,
    ray_base: Vec2,
    viewport_width: f32,
}

impl Camera {
    pub(crate) fn new() -> Self {
        Camera {
            pos: Vec2::new(0_f32, 0_f32),
            direction: Vec2::new(0_f32, 0_f32),
            projection_distance: 0_f32,
            ray_base: Vec2::new(0_f32, 0_f32),
            viewport_width: 0_f32,
        }
    }

    pub(crate) fn sync(&mut self, screen: &Screen, fov: f32) {
        self.viewport_width = screen.height();
        self.projection_distance =
            self.viewport_width * (fov.to_radians() / 2_f32).to_radians().tan() / 2_f32;
    }

    pub(crate) fn sync_pos(&mut self, x: f32, y: f32, yaw: f32) {
        self.pos = Vec2::new(x, y);

        let yaw_rad = yaw.to_radians();
        self.direction = Vec2::new(
            yaw_rad.sin() * self.projection_distance,
            yaw_rad.cos() * self.projection_distance,
        );

        let ray_base_angel = yaw_rad - NINETY_DEGREES_IN_RAD;
        self.ray_base = Vec2::new(ray_base_angel.sin(), ray_base_angel.cos());
    }

    pub(crate) fn calc_column(
        &self,
        screen: &Screen,
        column: f32,
        world: &[u8],
        world_width: i32,
    ) -> (f32, f32, Side, u8) {
        let ray_direction = self.ray_direction(screen, column);
        let (pos, side, distance) = self.cast_ray(ray_direction, world, world_width);

        let line_height = screen.height() / distance;

        let mut draw_start = -line_height / 2_f32 + screen.y_middle();
        let mut draw_end = line_height / 2_f32 + screen.y_middle();
        let material = world[(pos.y * world_width + pos.x) as usize];

        if draw_start < 0_f32 {
            draw_start = 0_f32;
        }

        if draw_end >= screen.height() {
            draw_end = screen.height() - 1_f32;
        }

        (draw_start, draw_end, side, material)
    }

    // https://lodev.org/cgtutor/raycasting.html
    // https://youtu.be/NbSee-XM7WA
    // https://permadi.com/1996/05/ray-casting-tutorial-table-of-contents/
    fn cast_ray(&self, direction: Vec2, world: &[u8], world_width: i32) -> (IVec2, Side, f32) {
        // current position
        let mut pos = IVec2::new(self.pos.x as i32, self.pos.y as i32);

        // distance from one to another x-/y-axis (unit), division with zero = infinity
        let delta = Vec2::new((1_f32 / direction.x).abs(), (1_f32 / direction.y).abs());

        // x and y direction to walk
        let step_x;
        let step_y;

        // distance from current pos to next x-/y-axis (unit)
        let mut side_x;
        let mut side_y;

        // wall hit at North - South or East - West
        let mut side;

        // calculating step_x/y and initial side_x/y
        if direction.x < 0_f32 {
            step_x = -1_i32;
            side_x = (self.pos.x - (pos.x as f32)) * delta.x;
        } else {
            step_x = 1_i32;
            side_x = ((pos.x as f32) + 1_f32 - self.pos.x) * delta.x;
        }

        if direction.y < 0_f32 {
            step_y = -1_i32;
            side_y = (self.pos.y - (pos.y as f32)) * delta.y;
        } else {
            step_y = 1_i32;
            side_y = (self.pos.y + 1_f32 - (pos.y as f32)) * delta.y;
        }

        // do-while hack
        while {
            if side_x < side_y {
                side_x += delta.x;
                pos.x += step_x;
                side = Side::EW;
            } else {
                side_y += delta.y;
                pos.y += step_y;
                side = Side::NS;
            }

            // condition: no semicolon ;)
            world[(pos.y * world_width + pos.x) as usize] == 0
        } {}

        // absolute distance that was casted
        let cast_distance = match side {
            Side::EW => side_x - delta.x,
            Side::NS => side_y - delta.y,
        };

        (pos, side, cast_distance)
    }

    pub(crate) fn ray_direction(&self, screen: &Screen, column: f32) -> Vec2 {
        // percentage of the current column/ray from -100% to 100%
        // where 0% is the middle and -100% the most left column/ray
        // ^ according to the viewport
        //   for the real screen the percentage can gow above and below (-)100%
        let ray_x_viewport = 2_f32 * column / self.viewport_width - 1_f32;
        let ray_x = (ray_x_viewport * screen.width()) / self.viewport_width;

        // This is a vector that points with 90 degrees to the left or right
        // depending of the above calculated percentage.
        //
        // `self.ray_base` is a vector that point with 90 degrees from the player
        // straight to the right till the end of the projection plane.
        let ray_base = Vec2::new(self.ray_base.x * ray_x, self.ray_base.y * ray_x);

        // a simple vector addition of the current direction and the above calculated
        // ray direction (left or right). This results in a vector that points from the
        // cameras position to the position of the projection plane of the current
        // pixel column.
        let direction = self.direction + ray_base;
        let direction_length = self.direction.length();

        // because the direction is going to be normalized every ray has the same direction
        // and only the ray for the middle (ray_x = 0) is able to reach the projection plane
        // missing_length in percent
        let missing_length =
            (direction_length - self.projection_distance) / self.projection_distance;

        let direction_x_norm = direction.x / direction_length;
        let direction_y_norm = direction.y / direction_length;

        Vec2::new(
            direction_x_norm + direction_x_norm * missing_length,
            direction_y_norm + direction_y_norm * missing_length,
        )
    }
}
