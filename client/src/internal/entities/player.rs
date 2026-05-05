use std::collections::HashSet;

use sdl3::{keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::internal::{common::Vector2, entities::Entity};

pub struct Player {
    bounds: Rect,
    speed: f32,
}

impl Entity for Player {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>, camera: Vector2) {
        let screen_x = self.bounds.x - camera.x as i32;
        let screen_y = self.bounds.y - camera.y as i32;

        let dest_rect = Rect::new(
            screen_x,
            screen_y,
            self.bounds.w as u32,
            self.bounds.h as u32,
        );
        canvas.set_draw_color(Color::RGB(100, 0, 0));
        canvas.fill_rect(dest_rect).unwrap();
    }

    fn update(self: &mut Self) {
        // World Bounds for Player
        // self.bounds.x = self.bounds.x.clamp(0, 2000 - self.bounds.w);
        // self.bounds.y = self.bounds.y.clamp(0, 2000 - self.bounds.h);
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            bounds: Rect::new(0, 0, 100, 100),
            speed: 5.0,
        }
    }

    pub fn move_player(self: &mut Self, keystate: HashSet<Scancode>) {
        for key in keystate {
            if key == Scancode::Right {
                self.bounds.x += self.speed as i32;
            }
            if key == Scancode::Left {
                self.bounds.x -= self.speed as i32;
            }
            if key == Scancode::Up {
                self.bounds.y -= self.speed as i32;
            }
            if key == Scancode::Down {
                self.bounds.y += self.speed as i32;
            }
        }
    }

    pub fn get_bounds(self: &mut Self) -> Rect {
        self.bounds
    }
}
