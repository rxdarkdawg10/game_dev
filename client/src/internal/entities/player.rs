use crate::internal::entities::Entity;

pub struct Player {}

impl Entity for Player {
    fn init(self: &Self) -> Self {
        todo!()
    }

    fn draw(self: &mut Self) {
        todo!()
    }

    fn update(self: &mut Self) {
        todo!()
    }
}

impl Player {
    pub fn new() -> Self {
        Player {}
    }
}
