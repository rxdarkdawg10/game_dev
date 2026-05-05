use std::collections::HashSet;

use sdl3::{keyboard::Scancode, pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::internal::{common::Vector2, entities::Entity};

pub struct Player {
    bounds: Rect,
    speed: f32,
}

impl Entity for Player {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(100, 0, 0));
        canvas.fill_rect(self.bounds).unwrap();
    }

    fn update(self: &mut Self, camera: Vector2) {
        self.bounds.x = self.bounds.x.clamp(0, 800 - self.bounds.w);
        self.bounds.y = self.bounds.y.clamp(0, 600 - self.bounds.h);

        self.bounds.x -= camera.x as i32;
        self.bounds.y -= camera.y as i32;
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            bounds: Rect::new(0, 0, 100, 100),
            speed: 10.0,
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
