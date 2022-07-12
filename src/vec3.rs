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
                pub fn cross(&self, other: Self) -> Self {
                    Self::new(
                        (self.y * other.z) + (-self.z * other.y),
                        (self.z * other.x) + (-self.x * other.z),
                        (self.x * other.y) + (-self.y * other.x),
                    )
                }

            }

            impl Vector for Vec3 {
                type Decimal = $t;
                type Bivec = $bv;
                type Rotor = $rot;

                #[inline]
                fn mag(&self) -> Self::Decimal {
                    self.mag_sq().sqrt()
                }

                #[inline]
                fn mag_sq(&self) -> Self::Decimal {
                    self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
                }

                #[inline]
                fn dot(&self, other: Self) -> Self::Decimal {
                    self.x * other.x + self.y * other.y + self.z * other.z
                }

                #[inline]
                fn wedge(&self, other: Self) -> Self::Bivec {
                    Self::Bivec::new(
                        (self.x * other.y) - (self.y * other.x),
                        (self.x * other.z) - (self.z * other.x),
                        (self.y * other.z) - (self.z * other.y),
                    )
                }

                #[inline]
                fn geom(&self, other: Self) -> Self::Rotor {
                    Self::Rotor::new(self.dot(other), self.wedge(other))
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
                fn dist_to_other(&self, other: Self) -> Self::Decimal {
                    self.to_other(other).mag()
                }

                #[inline]
                fn angle_between(&self, other: Self) -> Self::Decimal {
                    self.normalized().dot(other.normalized()).acos()
                }

                #[inline]
                fn angle_between_normal(&self, normal: Self) -> Self::Decimal {
                    self.dot(normal).acos()
                }

                #[inline]
                fn lerp(&self, to: Self, t: Self::Decimal) -> Self {
                    (1.0 - t) * *self + t * to
                }

                #[inline]
                fn slerp(&self, to: Self, t: Self::Decimal) -> Self {
                    let theta = self.angle_between(to);
                    let self_coef = ((1.0 - t) * theta).sin() / theta.sin();
                    let to_coef = (t * theta).sin() / theta.sin();
                    self_coef * *self + to_coef * to
                }

                #[inline]
                fn nlerp(&self, to: Self, t: Self::Decimal) -> Self {
                    self.lerp(to, t).normalized()
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
                    self.x += rhs.x;
                    self.y += rhs.y;
                    self.z += rhs.z;
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
                    self.x -= rhs.x;
                    self.y -= rhs.y;
                    self.z -= rhs.z;
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
                    self.z *= rhs;
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
                    self.z /= rhs;
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
                        2 => &self.z,
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
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl Into<[$t; 3]> for $nam {
                #[inline]
                fn into(self) -> [$t; 3] {
                    [self.x, self.y, self.z]
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

            impl From<$v4> for $nam {
                #[inline]
                fn from(vec4: $v4) -> Self {
                    Self::new(vec4.x, vec4.y, vec4.z)
                }
            }
        )+
    };
}

impl_vec3![(f32, Vec3, Vec2, Vec4, Bivec3, Rot3)];
