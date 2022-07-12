use crate::*;

pub struct Mat3 {
    pub cols: [Vec3; 3],
}

impl Mat3 {
    pub fn new(col1: Vec3, col2: Vec3, col3: Vec3) -> Self {
        Self {
            cols: [col1, col2, col3],
        }
    }
}
