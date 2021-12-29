use std::thread;

use macroquad::prelude::{
    draw_line, draw_text, get_fps, get_frame_time, is_key_down, next_frame, screen_height,
    screen_width, Color, Conf, KeyCode, BLACK, BLUE, GREEN, PURPLE, RED, WHITE, YELLOW,
};
use macroquad::Window;

use crate::camera::{Camera, Side};
use crate::screen::Screen;
use crate::Side::NS;

mod camera;
mod mini_map;
mod screen;

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
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 2, 2, 2, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 2, 2, 0, 2, 2, 0, 0, 0, 0, 3, 0, 3, 0, 3, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 0, 0, 0, 0, 5, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 0, 4, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 0, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 4, 4, 4, 4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // y row
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // y row
];

fn rotate(yaw: f32, step: f32) -> f32 {
    (yaw + ((get_frame_time() / step) * 100.0) + 360.0) % 360.0
}

fn walk(x: f32, y: f32, yaw: f32, step: f32, max_x: f32, max_y: f32) -> (f32, f32) {
    let dx = yaw.to_radians().sin() * ((get_frame_time() / step) * 75.0) * 0.1;
    let dy = yaw.to_radians().cos() * ((get_frame_time() / step) * 75.0) * 0.1;

    (
        (max_x - 0.1).min(x + dx).max(1_f32),
        (max_y - 0.1).min(y + dy).max(1_f32),
    )
}

async fn draw() {
    let mut x = 22_f32;
    let mut y = 12_f32;
    let mut yaw = -90_f32;
    let fov = 60_f32;

    let mut old_fov = fov;

    let mut old_x = x + 1_f32;
    let mut old_y = y + 1_f32;
    let mut old_yaw = yaw + 1_f32;

    const WALK_SPEED: f32 = 1_f32;
    const TURN_SPEED: f32 = 1_f32;

    let mut screen = Screen::new();
    let mut camera = Camera::new();

    loop {
        if screen.sync(screen_width(), screen_height()) || old_fov != fov {
            old_fov = fov;
            camera.sync(&screen, fov);
        }

        if old_x != x || old_y != y || old_yaw != yaw {
            old_x = x;
            old_y = y;
            old_yaw = yaw;
            camera.sync_pos(x, y, yaw);
        }

        if is_key_down(KeyCode::Left) {
            yaw = rotate(yaw, TURN_SPEED);
        }

        if is_key_down(KeyCode::Right) {
            yaw = rotate(yaw, -TURN_SPEED);
        }

        if is_key_down(KeyCode::W) {
            let (x1, y1) = walk(x, y, yaw, WALK_SPEED, 23_f32, 23_f32);
            x = x1;
            y = y1;
        }

        for column in 0..(screen_width() as u32) {
            let (draw_start, draw_end, side, material) =
                camera.calc_column(&screen, column as f32, &MAP, MAP_WIDTH as i32);

            let mut color = match material {
                1 => RED,
                2 => GREEN,
                3 => BLUE,
                4 => WHITE,
                5 => YELLOW,
                _ => PURPLE,
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

        // for column in 0..(screen_width() as u32) {
        //        ray_pos 0 is the middle of the screen
        // let direction = camera.ray_direction(column as f32);
        // draw_line(
        //     10.0,
        //     10.0,
        //     60.0 - direction.x * 50.0,
        //     60.0 - direction.y * 50.0,
        //     1.0,
        //     WHITE,
        // );
        // }

        let xy = format!("X/Y: {:.1}/{:.1}", x, y);
        draw_text(xy.as_str(), 12_f32, 32_f32, 20_f32, BLACK);
        draw_text(xy.as_str(), 10_f32, 30_f32, 20_f32, WHITE);

        let yaw = format!("YAW: {:.1}°", yaw);
        draw_text(yaw.as_str(), 12_f32, 62_f32, 20_f32, BLACK);
        draw_text(yaw.as_str(), 10_f32, 60_f32, 20_f32, WHITE);

        let fps = format!("FPS: {}", get_fps());
        draw_text(fps.as_str(), 12_f32, 92_f32, 20_f32, BLACK);
        draw_text(fps.as_str(), 10_f32, 90_f32, 20_f32, WHITE);

        next_frame().await
    }
}
