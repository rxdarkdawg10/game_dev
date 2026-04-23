use crate::internal::{
    entities::{Entity, player::Player},
    scenes::SCENES,
};

pub struct world {
    _type: SCENES,
    _players: Vec<Player>,
    _objects: Vec<Box<dyn Entity>>,
}
