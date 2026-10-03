use sdl3::{render::Canvas, video::Window};

use crate::internal::{
    common::{Vector2, render_spritesheet},
    entities::{self, Entity},
    scenes::world::World,
    sprites::{SPRITETYPE, Sprite},
};

pub struct UI {
    health_elem: Sprite,
}

impl UI {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            health_elem: Sprite::new(
                SPRITETYPE::HEALTH,
                5,
                2.5,
                Vector2::new(0.0, 0.0),
                Vector2::new(width as f32, 0.0),
            ),
        }
    }

    pub fn draw(self: &mut Self, canvas: &mut Canvas<Window>, camera: Vector2) {
        self.health_elem.sprite_src.x = 32.0 * self.health_elem.curr_frame as f32;

        render_spritesheet(
            self.health_elem.sprite_src,
            self.health_elem.sprite_size,
            self.health_elem.sprite_location,
            canvas,
        )
        .unwrap();
    }

    pub fn update(self: &mut Self, world: &mut World) {
        let quad = match world.player.get_health() {
            1..=25 => 3,
            26..=50 => 2,
            51..=75 => 1,
            76..=100 => 0,
            _ => 4
        };

        self.health_elem.curr_frame = quad;
    }
}
