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
}
