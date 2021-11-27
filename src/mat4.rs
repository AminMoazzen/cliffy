use crate::*;

macro_rules! impl_mat4 {
    [$(($t:ident, $nam:ident, $v3:ident, $v4:ident, $bv:ident, $rot:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub cols: [$v4; 4],
            }

            impl $nam{
                #[inline]
                pub fn new(col1: $v4, col2: $v4, col3: $v4, col4: $v4) -> Self {
                    Self {
                        cols: [col1, col2, col3, col4],
                    }
                }

                #[inline]
                pub fn identity() -> Self{
                    Self {
                        cols: [
                                $v4{x: 1.0, y: 0.0, z: 0.0, w: 0.0},
                                $v4{x: 0.0, y: 1.0, z: 0.0, w: 0.0},
                                $v4{x: 0.0, y: 0.0, z: 1.0, w: 0.0},
                                $v4{x: 0.0, y: 0.0, z: 0.0, w: 1.0},
                            ]
                    }
                }
            }
        )+
    };
}

impl_mat4![(f32, Mat4, Vec3, Vec4, Bivec3, Rot3)];
