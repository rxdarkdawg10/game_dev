use std::collections::HashSet;

use sdl3::{
    keyboard::{KeyboardState, Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
};

use crate::internal::{common::Vector2, entities::Entity, system::camera::Camera};

pub struct Player {
    bounds: Rect,
    _camera: Camera,
}

impl Entity for Player {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(100, 0, 0));
        canvas.fill_rect(self.bounds).unwrap();
    }

    fn update(self: &mut Self) {
        self._camera.update(self.bounds);
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            bounds: Rect::new(0, 0, 100, 100),
            _camera: Camera::new(Vector2::new(0.0, 0.0), 800, 600),
        }
    }

    pub fn move_player(self: &mut Self, keystate: HashSet<Scancode>) {
        for key in keystate {
            if key == Scancode::Right {
                self.bounds.x += 5;
            }
            if key == Scancode::Left {
                self.bounds.x -= 5;
            }
            if key == Scancode::Up {
                self.bounds.y -= 5;
            }
            if key == Scancode::Down {
                self.bounds.y += 5;
            }
        }
    }
}
