use sdl3::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::internal::common::Vector2;

pub struct Camera {
    position: Vector2,
    width: i32,
    height: i32,
    lerp: f32,
}

impl Camera {
    pub fn new(pos: Vector2, width: i32, height: i32) -> Self {
        Camera {
            position: pos,
            width: width,
            height: height,
            lerp: 1.0,
        }
    }

    pub fn get_position(self: &mut Self) -> Vector2 {
        self.position
    }

    pub fn update(self: &mut Self, player: Rect) {
        let target_x = player.x as f32 + (player.w as f32 / 2.0) - (800.0 / 2.0);
        let target_y = player.y as f32 + (player.h as f32 / 2.0) - (600.0 / 2.0);

        self.position.x += (target_x - self.position.x) * self.lerp;
        self.position.y += (target_y - self.position.y) * self.lerp;
    }

    pub fn rect(self: &Self) -> Rect {
        Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            self.width as u32,
            self.height as u32,
        )
    }
}
