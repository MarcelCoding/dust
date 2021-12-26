use std::thread;

use macroquad::prelude::{
    draw_line, is_key_down, next_frame, screen_height, screen_width, Color, Conf, KeyCode, Vec2,
    BLUE, GREEN, RED, WHITE, YELLOW,
};
use macroquad::Window;

use crate::camera::{Camera, Side};
use crate::Side::{EW, NS};

mod camera;
mod mini_map;

pub fn open_window() {
    thread::spawn(|| {
        Window::from_config(
            Conf {
                sample_count: 4,
                window_title: "Dust".to_string(),
                high_dpi: true,
                ..Default::default()
            },
            draw(),
        );
    });
}

pub(crate) const MAP_WIDTH: u16 = 24;
pub(crate) const MAP_HEIGHT: u16 = 24;

pub(crate) const MAP: [u8; (MAP_WIDTH * MAP_HEIGHT) as usize] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0,
    0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3,
    0, 0, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0,
    4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
];

fn rotate(yaw: f32, step: f32) -> f32 {
    (yaw + step + 360.0) % 360.0
}

fn walk(x: f32, y: f32, yaw: f32, step: f32, max_x: f32, max_y: f32) -> (f32, f32) {
    let dx = yaw.to_radians().sin() * step * 0.1;
    let dy = yaw.to_radians().cos() * step * 0.1;

    (
        (max_x - 1_f32).min(x + dx).max(2_f32),
        (max_y - 1_f32).min(y + dy).max(2_f32),
    )
}

async fn draw() {
    let mut x = 22_f32;
    let mut y = 12_f32;
    let mut yaw = -90_f32;
    let fov = 60_f32;

    loop {
        if is_key_down(KeyCode::Left) {
            yaw = rotate(yaw, 1_f32)
        }

        if is_key_down(KeyCode::Right) {
            yaw = rotate(yaw, -1_f32);
        }

        if is_key_down(KeyCode::W) {
            let (x1, y1) = walk(x, y, yaw, 1_f32, 23_f32, 23_f32);
            x = x1;
            y = y1;
        }

        let camera = Camera::new(x, y, yaw, fov, screen_width(), screen_height()); // 0 -> +x, 90 -> y+

        for column in 0..(screen_width() as u32) {
            // ray_pos 0 is the middle of the screen
            // let ray_pos = ray as f32;
            // let (ray_vec, dis_offset) = camera.ray_vec(sw, ray_pos);
            //
            // let (x, y, distance, side) = find_wall(&camera, ray_vec);
            //
            // let height = sh / (distance / dis_offset);
            //
            // let mut start = -height / 2.0 + sh / 2.0;
            // let mut end = height / 2.0 + sh / 2.0;
            //
            // if start < 0.0 {
            //     start = 0.0
            // }
            //
            // if end >= sh {
            //     end = sh - 1.0
            // }

            let (draw_start, draw_end, side, material) =
                camera.calc_column(column as f32, &MAP, MAP_WIDTH as i32);

            let mut color = match material {
                1 => RED,
                2 => GREEN,
                3 => BLUE,
                4 => WHITE,
                _ => YELLOW,
            };

            if side == NS {
                // cheapest way to do shading
                // i would do a more manual approach later
                color = Color {
                    a: color.a,
                    r: color.r / 2.0,
                    g: color.g / 2.0,
                    b: color.b / 2.0,
                }
            }

            draw_line(
                column as f32,
                draw_start,
                column as f32,
                draw_end,
                1.0,
                color,
            );
        }

        for column in 0..(screen_width() as u32) {
            //    ray_pos 0 is the middle of the screen
            let direction = camera.ray_direction(column as f32);
            draw_line(
                10.0,
                10.0,
                60.0 - direction.x * 50.0,
                60.0 - direction.y * 50.0,
                1.0,
                WHITE,
            );
        }
        next_frame().await
    }
}
