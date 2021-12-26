use std::f32::consts::PI;

use macroquad::prelude::{IVec2, Vec2};

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
    width: f32,
    height: f32,
}

impl Camera {
    pub(crate) fn new(x: f32, y: f32, yaw: f32, fov: f32, width: f32, height: f32) -> Self {
        let yaw_rad = yaw.to_radians();
        let projection_plane_distance =
            width * (fov.to_radians() / 2_f32).to_radians().tan() / 2_f32;
        let ray_base_angel = yaw_rad - NINETY_DEGREES_IN_RAD;

        let pos = Vec2::new(x, y);
        let direction = Vec2::new(
            yaw_rad.sin() * projection_plane_distance,
            yaw_rad.cos() * projection_plane_distance,
        );
        let projection_distance = direction.length();
        let ray_base = Vec2::new(ray_base_angel.sin(), ray_base_angel.cos());

        Camera {
            pos,
            direction,
            projection_distance,
            ray_base,
            width,
            height,
        }
    }

    pub(crate) fn calc_column(
        &self,
        column: f32,
        world: &[u8],
        world_width: i32,
    ) -> (f32, f32, Side, u8) {
        let ray_direction = self.ray_direction(column);
        let (pos, side, distance) = self.cast_ray(ray_direction, world, world_width);

        let line_height = self.height / distance;

        let mut draw_start = -line_height / 2_f32 + self.height / 2_f32;
        let mut draw_end = line_height / 2_f32 + self.height / 2_f32;
        let material = world[(pos.y * world_width + pos.x) as usize];

        if draw_start < 0_f32 {
            draw_start = 0_f32;
        }

        if draw_end >= self.height {
            draw_end = self.height - 1_f32;
        }

        (draw_start, draw_end, side, material)
    }

    // ray between -x and x
    // pub(crate) fn ray_vec(&self, sw: f32, ray: f32) -> (Vec2, f32) {
    //     let fov_offset = self.yaw - (self.fov / 2.0);
    //     let rel_ray_angel = ray / sw * self.fov;
    //     let ray_angel = (fov_offset + rel_ray_angel).to_radians();
    //
    //     let fov_plane_dis = (self.fov / 2.0).to_radians().tan() * sw / 2.0;
    //
    //     let x = (ray - sw / 2.0).hypot(fov_plane_dis.abs());
    //
    //     // println!("{} {} {} {}", x , fov_plane_dis,x-fov_plane_dis, (x-fov_plane_dis)/x);
    //
    //     (
    //         Vec2::new(ray_angel.sin(), ray_angel.cos()),
    //         1.0 / (rel_ray_angel - self.fov / 2.0).to_radians().cos(),
    //         // x - fov_plane_dis/x
    //     )
    // }

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
        /*let cast_distance = match side {
            Side::EW => side_x - delta.x,
            Side::NS => side_y - delta.y,
        };*/

        let cast_distance = match side {
            Side::EW => side_x - delta.x,
            Side::NS => side_y - delta.y,
        };

        (pos, side, cast_distance)
    }

    pub(crate) fn ray_direction(&self, column: f32) -> Vec2 {
        // percentage of the current column/ray from -100% to 100%
        // where 0% is the middle and -100% the most left column/ray
        let ray_x = 2_f32 * column / self.width - 1_f32;

        // This is a vector that points with 90 degrees to the left or right
        // depending of the obove calculated percentage.
        //
        // `self.ray_base` is a vector that point with 90 degrees from the player
        // straight to the right till the end of the projection plane.
        let ray_base = Vec2::new(self.ray_base.x * ray_x, self.ray_base.y * ray_x);

        // a simple vector addition of the current direction and the above calculated
        // ray direction (left or right). This results in a vector that points from the
        // cameras position to the position of the projection plane of the current
        // pixel column.
        let direction = self.direction + ray_base;

        // because the direction is going to be normalized every ray has the same direction
        // and only the reay for the middle (ray_x = 0) is able to reach the projection plane
        // msissing_length in percent
        let missing_length =
            (direction.length() - self.projection_distance) / self.projection_distance;

        let normalized_direction = direction.normalize();

        Vec2::new(
            normalized_direction.x + normalized_direction.x * missing_length,
            normalized_direction.y + normalized_direction.y * missing_length,
        )
    }
}
