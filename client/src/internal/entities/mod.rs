use sdl3::{render::Canvas, video::Window};

use crate::internal::common::Vector2;

pub mod player;

pub trait Entity {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>);
    fn update(self: &mut Self, camera: Vector2);
}
