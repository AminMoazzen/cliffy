use crate::*;

pub struct Mat2 {
    pub cols: [Vec2; 2],
}

impl Mat2 {
    pub fn new(col1: Vec2, col2: Vec2) -> Self {
        Self { cols: [col1, col2] }
    }
}
