use sdl3::{render::Canvas, video::Window};

use crate::internal::common::Vector2;

pub mod player;

pub trait Entity {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>, camera: Vector2);
    fn update(self: &mut Self);
}
