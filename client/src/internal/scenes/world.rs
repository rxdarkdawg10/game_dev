use crate::internal::{
    entities::{Entity, player::Player},
    scenes::SCENES,
};

pub struct World {
    _type: SCENES,
    _players: Vec<Player>,
    _objects: Vec<Box<dyn Entity>>,
}
