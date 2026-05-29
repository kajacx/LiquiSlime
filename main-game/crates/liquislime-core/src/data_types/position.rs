use crate::TilePosition;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn to_tile_position(self) -> TilePosition {
        TilePosition::new(self.x.floor() as i32, self.y.floor() as i32)
    }
}
