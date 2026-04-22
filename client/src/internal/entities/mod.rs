pub mod player;

pub trait Entity {
    fn init(self: &Self) -> Self;
    fn draw(self: &mut Self);
    fn update(self: &mut Self);
}
