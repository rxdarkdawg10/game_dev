use crate::internal::common::Vector2;

pub struct Camera {
    position: Vector2,
    width: i32,
    height: i32,
}

impl Camera {
    pub fn new(pos: Vector2, width: i32, height: i32) -> Self {
        Camera {
            position: pos,
            width: width,
            height: height,
        }
    }
}
