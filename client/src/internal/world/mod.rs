use std::cell::RefCell;

use crate::internal::{common::Vector2, entities::player::Player, system::camera::Camera};

pub struct World<'a> {
    pub camera: Camera,
    pub player: &'a mut Player,
    pub players: Vec<&'a Player>,
}

impl<'a> World<'a> {
    pub fn new(player: &'a mut Player) -> Self {
        World {
            camera: Camera::new(Vector2::new(0.0, 0.0), 800, 600),
            players: Vec::new(),
            player: player,
        }
    }
}
