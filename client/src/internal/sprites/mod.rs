use crate::internal::common::Vector2;

pub enum SPRITETYPE {
    HEALTH,
}

pub struct Sprite {
    _type: SPRITETYPE,
    pub frames: usize,
    pub curr_frame: usize,
    pub sprite_size: f32,
    pub sprite_src: Vector2,
    pub sprite_location: Vector2,
}

impl Sprite {
    pub fn new(
        sprite_type: SPRITETYPE,
        frames: usize,
        sprite_size: f32,
        sprite_src: Vector2,
        sprite_location: Vector2,
    ) -> Self {
        Self {
            _type: sprite_type,
            frames: frames,
            sprite_size: sprite_size,
            sprite_location: sprite_location,
            sprite_src: sprite_src,
            curr_frame: 0,
        }
    }
}
