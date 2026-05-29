use derive_more::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Add, Sub)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

impl TilePosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
