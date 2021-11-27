use crate::*;

macro_rules! impl_mat3 {
    [$(($t:ident, $nam:ident, $v2:ident, $v3:ident, $v4:ident, $bv:ident, $rot:ident, $mat4:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub cols: [$v3; 3],
            }

            impl $nam{
                #[inline]
                pub fn new(col1: $v3, col2: $v3, col3: $v3) -> Self {
                    Self {
                        cols: [col1, col2, col3],
                    }
                }

                #[inline]
                pub fn identity() -> Self{
                    Self {
                        cols: [
                                $v3{x: 1.0, y: 0.0, z: 0.0},
                                $v3{x: 0.0, y: 1.0, z: 0.0},
                                $v3{x: 0.0, y: 0.0, z: 1.0},
                            ]
                    }
                }
            }
        )+
    };
}

impl_mat3![(f32, Mat3, Vec2, Vec3, Vec4, Bivec3, Rot3, Mat4)];
