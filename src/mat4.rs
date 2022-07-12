use crate::*;

pub struct Mat4 {
    pub cols: [Vec4; 4],
}

impl Mat4 {
    pub fn new(col1: Vec4, col2: Vec4, col3: Vec4, col4: Vec4) -> Self {
        Self {
            cols: [col1, col2, col3, col4],
        }
    }
}
