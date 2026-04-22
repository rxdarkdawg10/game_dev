use sdl3::{render::Canvas, video::Window};

pub mod player;

pub trait Entity {
    fn draw(self: &mut Self, canvas: &mut Canvas<Window>);
    fn update(self: &mut Self);
}
