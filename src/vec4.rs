use crate::*;
use std::ops::*;

macro_rules! impl_vec4 {
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

                #[inline]
                pub fn uni(val: $t) -> Self {
                    Self::new(val, val, val, val)
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::uni(0.0)
                }

                #[inline]
                pub fn one() -> Self {
                    Self::uni(1.0)
                }

            }
            #[inline]
            fn mag(&self) -> Self::$t {
                self.mag_sq().sqrt()
            }
            #[inline]
            fn mag_sq(&self) -> Self::$t {
                self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)
            }
            #[inline]
            fn dot(&self, other: Self) -> Self::$t {
                self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
            }
            #[inline]
            fn normalize(&mut self) {
                let mag = self.mag();
                *self /= mag;
            }
            #[inline]
            fn normalized(&self) -> Self {
                let mut v = self.clone();
                v.normalize();
                v
            }
            #[inline]
            fn project(&mut self, other: Self) {
                *self = (self.dot(other) / other.mag_sq()) * other;
            }
            #[inline]
            fn projected(&self, other: Self) -> Self {
                let mut v = self.clone();
                v. project(other);
                v
            }
            #[inline]
            fn reject(&mut self, other: Self) {
                // self = self - self.project(other)
                *self -= (self.dot(other) / other.mag_sq()) * other;
            }
            #[inline]
            fn rejected(&self, other: Self) -> Self {
                let mut v = self.clone();
                v. reject(other);
                v
            }
            #[inline]
            fn reflect(&mut self, other: Self) {
                // self = self - 2 * self.project(other)
                *self -= 2.0 * (self.dot(other) / other.mag_sq()) * other;
            }
            #[inline]
            fn reflected(&self, other: Self) -> Self {
                let mut v = self.clone();
                v. reflect(other);
                v
            }
            #[inline]
            fn reflect_normal(&mut self, normal: Self) {
                // self = self - 2 * self.project(normal)
                *self -= 2.0 * self.dot(normal) * normal;
            }
            #[inline]
            fn reflected_normal(&self, normal: Self) -> Self {
                let mut v = self.clone();
                v. reflect_normal(normal);
                v
            }
            #[inline]
            fn to_other(&self, other: Self) -> Self {
                other - *self
            }
            #[inline]
            fn dist_to_other(&self, other: Self) -> Self::$t {
                self.to_other(other).mag()
            }
            #[inline]
            fn lerp(&self, to: Self, t: Self::$t) -> Self {
                (1.0 - t) * *self + t * to
            }
            #[inline]
            fn slerp(&self, to: Self, t: Self::$t) -> Self {
                let theta = self.angle_between(to);
                let self_coef = ((1.0 - t) * theta).sin() / theta.sin();
                let to_coef = (t * theta).sin() / theta.sin();
                self_coef * *self + to_coef * to
            }
            #[inline]
            fn nlerp(&self, to: Self, t: Self::$t) -> Self {
                self.lerp(to, t).normalized()
            }


            impl Add for $nam {
                type Output = $nam;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.x += rhs.x;
                    self.y += rhs.y;
                    self.z += rhs.z;
                    self.w += rhs.w;
                }
            }

            impl Sub for $nam {
                type Output = $nam;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.x -= rhs.x;
                    self.y -= rhs.y;
                    self.z -= rhs.z;
                    self.w -= rhs.w;
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
                    Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
                }
            }

            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.x *= rhs;
                    self.y *= rhs;
                    self.z *= rhs;
                    self.w *= rhs;
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> Self::Output {
                    $nam::new(self * rhs.x, self * rhs.y, self * rhs.z, self * rhs.w)
                }
            }

            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> Self::Output {
                    Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
                }
            }

            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.x /= rhs;
                    self.y /= rhs;
                    self.z /= rhs;
                    self.w /= rhs;
                }
            }

            impl Div<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $nam) -> Self::Output {
                    $nam::new(self / rhs.x, self / rhs.y, self / rhs.z, self / rhs.w)
                }
            }

            impl Index<usize> for $nam {
                type Output =  $t;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        2 => &self.z,
                        2 => &self.w,
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl IndexMut<usize> for $nam {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        2 => &mut self.z,
                        3 => &mut self.z,
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl Into<[$t; 4]> for $nam {
                #[inline]
                fn into(self) -> [$t; 4] {
                    [self.x, self.y, self.z, self.w]
                }
            }

            impl From<[$t; 4]> for $nam {
                #[inline]
                fn from(comps: [$t; 4]) -> Self {
                    Self::new(comps[0], comps[1], comps[2], comps[3])
                }
            }

            impl From<&[$t; 4]> for $nam {
                #[inline]
                fn from(comps: &[$t; 4]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<&mut [$t; 4]> for $nam {
                #[inline]
                fn from(comps: &mut [$t; 4]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<($t, $t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: ($t, $t, $t, $t)) -> Self {
                    Self::new(comps.0, comps.1, comps.2, comps.3)
                }
            }

            impl From<&($t, $t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: &($t, $t, $t, $t)) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<$nam> for ($t, $t) {
                #[inline]
                fn from(v: $nam) -> Self {
                    (v.x, v.y)
                }
            }

            impl Into<$v3> for $nam {
                #[inline]
                fn into(self) -> $v3 {
                    $v3::new(self.x, self.y, self.z)
                }
            }

            impl From<$v3> for $nam {
                #[inline]
                fn from(vec4: $v3) -> Self {
                    Self::new(vec4.x, vec4.y, vec4.z, 0.0)
                }
            }

            impl Into<$v2> for $nam {
                #[inline]
                fn into(self) -> $v2 {
                    $v2::new(self.x, self.y)
                }
            }

            impl From<$v2> for $nam {
                #[inline]
                fn from(vec2: $v2) -> Self {
                    Self::new(vec2.x, vec2.y, 0.0, 0.0)
                }
            }
        )+
    };
}

impl_vec4![(f32, Vec4, Vec2, Vec3)];
