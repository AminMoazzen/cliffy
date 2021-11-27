use crate::*;

macro_rules! impl_vec3 {
    [$(($t:ident, $nam:ident, $v2:ident, $v3:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub x: $t,
                pub y: $t,
                pub z: $t,
                pub w: $t,
            }

            impl $nam{
                #[inline]
                pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                    Self { x, y, z, w }
                }
            }
        )+
    };
}

impl_vec3![(f32, Vec4, Vec2, Vec3)];
