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

    pub fn get_position(self: &mut Self) -> Vector2 {
        self.position
    }

    pub fn update(self: &mut Self, player: Rect) {
        // if self.position.x < 0.0 {
        //     self.position.x = 0.0;
        // }

        // if self.position.y < 0.0 {
        //     self.position.y = 0.0;
        // }

        // if self.position.x > (600 - self.width) as f32 {
        //     self.position.x = (600 - self.width) as f32;
        // }

        // if self.position.y > (800 - self.height) as f32 {
        //     self.position.y = (600 - self.height) as f32;
        // }

        self.position.x = (player.x + player.w / 2 - self.width / 2) as f32;
        self.position.y = (player.y + player.h / 2 - self.height / 2) as f32;

        // self.position.x = self.position.x.clamp(0.0, 800.0 - 800.0);
        // self.position.y = self.position.y.clamp(0.0, 800.0 - 800.0);
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
