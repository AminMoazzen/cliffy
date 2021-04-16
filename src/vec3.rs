use crate::*;
use std::ops::*;

macro_rules! impl_vec3 {
    [$(($t:ident, $nam:ident, $v2:ident, $v4:ident, $bv:ident, $rot:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub x: $t,
                pub y: $t,
                pub z: $t,
            }

            impl $nam{
                #[inline]
                pub fn new(x: $t, y: $t, z: $t) -> Self {
                    Self { x, y, z }
                }

                #[inline]
                pub fn uni(val: $t) -> Self {
                    Self::new(val, val, val)
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::uni(0.0)
                }

                #[inline]
                pub fn one() -> Self {
                    Self::uni(1.0)
                }

                #[inline]
                pub fn up() -> Self {
                    Self::new(0.0, 1.0, 0.0)
                }

                #[inline]
                pub fn down() -> Self {
                    Self::new(0.0, -1.0, 0.0)
                }

                #[inline]
                pub fn right() -> Self {
                    Self::new(1.0, 0.0, 0.0)
                }

                #[inline]
                pub fn left() -> Self {
                    Self::new(-1.0, 0.0, 0.0)
                }

                #[inline]
                pub fn forward() -> Self {
                    Self::new(0.0, 0.0, 1.0)
                }

                #[inline]
                pub fn back() -> Self {
                    Self::new(0.0, 0.0, -1.0)
                }

                #[inline]
                pub fn mag(&self) -> $t {
                    self.mag_sq().sqrt()
                }

                #[inline]
                pub fn mag_sq(&self) -> $t {
                    self.x.powi(2) + self.y.powi(2)
                }

                #[inline]
                pub fn dot(&self, other: Self) -> $t {
                    self.x * other.x + self.y * other.y
                }

                #[inline]
                pub fn wedge(&self, other: Self) -> $bv {
                    $bv::new(self.x * other.y - self.y * other.x)
                }

                #[inline]
                pub fn geom(&self, other: Self) -> $rot {
                    $rot::new(self.dot(other), self.wedge(other))
                }

                #[inline]
                pub fn normalize(&mut self) {
                    let mag = self.mag();
                    *self /= mag;
                }

                #[inline]
                pub fn normalized(&self) -> Self {
                    let mut v = self.clone();
                    v.normalize();
                    v
                }

                #[inline]
                pub fn inverse(&self) -> Self {
                    *self / self.mag_sq()
                }

                #[inline]
                pub fn reflect(&self, other: Self) -> Self {
                    // other.inverse().geom(self) * other
                    todo!()
                }

                #[inline]
                pub fn reject(&self, other: Self) -> Self {
                    // self.wedge(other) * other.inverse()
                    todo!()
                }

                #[inline]
                pub fn project(&self, other: Self) -> Self {
                    // self.dot(other) * ohter.inverse()
                    // (self.dot(other) / other.dot(other)) * other
                    todo!()
                }

                #[inline]
                pub fn to(&self, dest: Self) -> Self {
                    dest - *self
                }

                #[inline]
                pub fn dist(&self, dest: Self) -> $t {
                    self.to(dest).mag()
                }

            }

            impl Add for $nam {
                type Output = $nam;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.x = self.x + rhs.x;
                    self.y = self.x + rhs.y;
                }
            }

            impl Sub for $nam {
                type Output = $nam;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.x = self.x - rhs.x;
                    self.y = self.x - rhs.y;
                }
            }

            impl Neg for $nam{
                type Output = $nam;

                #[inline]
                fn neg(self) -> $nam {
                    self * -1.0
                }
            }

            impl Mul<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $t) -> Self::Output {
                    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
                }
            }

            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.x *= rhs;
                    self.y *= rhs;
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> Self::Output {
                    $nam::new(self * rhs.x, self * rhs.y, self * rhs.z)
                }
            }

            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> Self::Output {
                    Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
                }
            }

            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.x /= rhs;
                    self.y /= rhs;
                }
            }

            impl Div<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $nam) -> Self::Output {
                    $nam::new(self / rhs.x, self / rhs.y, self / rhs.z)
                }
            }

            impl Index<usize> for $nam {
                type Output =  $t;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl IndexMut<usize> for $nam {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl Into<[$t; 2]> for $nam {
                #[inline]
                fn into(self) -> [$t; 2] {
                    [self.x, self.y]
                }
            }

            impl From<[$t; 3]> for $nam {
                #[inline]
                fn from(comps: [$t; 3]) -> Self {
                    Self::new(comps[0], comps[1], comps[2])
                }
            }

            impl From<&[$t; 3]> for $nam {
                #[inline]
                fn from(comps: &[$t; 3]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<&mut [$t; 3]> for $nam {
                #[inline]
                fn from(comps: &mut [$t; 3]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<($t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: ($t, $t, $t)) -> Self {
                    Self::new(comps.0, comps.1, comps.2)
                }
            }

            impl From<&($t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: &($t, $t, $t)) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<$nam> for ($t, $t) {
                #[inline]
                fn from(v: $nam) -> Self {
                    (v.x, v.y)
                }
            }

            impl Into<$v4> for $nam {
                #[inline]
                fn into(self) -> $v4 {
                    $v4::new(self.x, self.y, 0.0, 0.0)
                }
            }

            impl From<$v4> for $nam {
                #[inline]
                fn from(vec4: $v4) -> Self {
                    Self::new(vec4.x, vec4.y, vec4.z)
                }
            }
        )+
    };
}

impl_vec3![(f32, Vec3, Vec2, Vec4, Bivec3, Rotor3)];
