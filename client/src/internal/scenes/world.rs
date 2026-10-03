use sdl3::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::internal::{
    common::Vector2,
    entities::{Entity, player::Player},
    scenes::SCENES,
    system::camera::Camera,
};

pub struct World<'a> {
    _type: SCENES,
    pub camera: Camera,
    pub player: &'a mut Player,
    _players: Vec<Player>,
    _objects: Vec<Box<dyn Entity>>,
}

impl<'a> World<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        World {
            _type: SCENES::WORLD,
            camera: Camera::new(Vector2::new(400.0, 300.0), 200, 200),
            _players: Vec::new(),
            player: player,
            _objects: Vec::new(),
        }
    }

    pub fn draw(self: &mut Self, canvas: &mut Canvas<Window>, camera: &Vector2) {
        let screen_x = 20 - camera.x as i32;
        let screen_y = 20 - camera.y as i32;

        let dest_rect = Rect::new(
            screen_x,
            screen_y,
            700,
            50,
        );
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(dest_rect).unwrap();
    }
}
