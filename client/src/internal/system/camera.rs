use sdl3::rect::Rect;

use crate::internal::common::Vector2;

pub struct Camera {
    position: Vector2,
    width: i32,
    height: i32,
}

impl Camera {
    pub fn new(pos: Vector2, width: i32, height: i32) -> Self {
        Camera {
            position: pos,
            width: width,
            height: height,
        }
    }

    pub fn update(self: &mut Self, rec: Rect) {
        self.position.x = (rec.x + rec.w / 2 - self.width / 2) as f32;
        self.position.y = (rec.y + rec.h / 2 - self.height / 2) as f32;
    }
}
