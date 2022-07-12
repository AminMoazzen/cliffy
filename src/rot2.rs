use crate::*;
use std::ops::*;

macro_rules! impl_rotor2 {
    [$(($t:ident, $nam:ident, $bv:ident, $v2:ident, $m2:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub s: $t,
                pub bv: $bv,
            }

            impl $nam {
                #[inline]
                pub fn new(s: $t, bv: $bv) -> Self {
                    Self {s, bv}
                }

                #[inline]
                pub fn identity() -> Self {
                    Self::new(1.0, $bv::zero())
                }

                #[inline]
                pub fn from_angle(angle: $t) -> Self {
                    let half_angle = angle / 2.0;
                    let (sin, cos) = half_angle.sin_cos();
                    Self::new(cos, $bv::new(-sin))
                }
            }

            impl Rotor for $nam {
                type Decimal = $t;
                type Vector = $v2;
                type Matrix = $m2;

                #[inline]
                fn mag(&self) -> Self::Decimal {
                    self.mag_sq().sqrt()
                }

                #[inline]
                fn mag_sq(&self) -> Self::Decimal {
                    self.s * self.s + self.bv.mag_sq()
                }

                #[inline]
                fn dot(&self, other: Self) -> Self::Decimal {
                    self.s * other.s + self.bv.dot(other.bv)
                }

                #[inline]
                fn normalize(&mut self) {
                    let mag = self.mag();
                    self.s /= mag;
                    self.bv.xy /= mag;
                }

                #[inline]
                fn normalized(&self) -> Self {
                    let mut r = self.clone();
                    r.normalize();
                    r
                }

                #[inline]
                fn reverse(&mut self) {
                    self.bv = -self.bv;
                }

                #[inline]
                fn reversed(&self) -> Self {
                    let mut r = self.clone();
                    r.reverse();
                    r
                }

                #[inline]
                fn rotate_by(&mut self, other: Self) {
                    let b = *self;
                    let a = other;
                    let sa2_plus_baxy2 = a.s.mul_add(a.s, a.bv.xy * a.bv.xy);

                    self.s = (a.s - b.s) * a.bv.xy * b.bv.xy
                        + b.s * sa2_plus_baxy2;
                    self.bv.xy = b.bv.xy * sa2_plus_baxy2;
                }

                #[inline]
                fn rotated_by(&self, other: Self) -> Self {
                    let mut r = self.clone();
                    r.rotate_by(other);
                    r
                }

                #[inline]
                fn rotate_vec(&self, vec: &mut Self::Vector) {
                    let fe1 = self.s * vec.x + self.bv.xy * vec.y;
                    let fe2 = self.s * vec.y - (self.bv.xy * vec.x);

                    vec.x = self.s * fe1 + self.bv.xy * fe2;
                    vec.y = self.s * fe2 - (self.bv.xy * fe1);
                }

                #[inline]
                fn into_matrix(&self) -> Self::Matrix {
                    let s2_minus_bxy2 = self.s * self.s - self.bv.xy * self.bv.xy;
                    let two_s_bxy = 2.0 * self.s * self.bv.xy;

                    $m2::new(
                        $v2::new(s2_minus_bxy2, -two_s_bxy),
                        $v2::new(two_s_bxy, s2_minus_bxy2))
                }
            }

            impl Add for $nam {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self {
                    Self::new(self.s + rhs.s, self.bv + rhs.bv)
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.s += rhs.s;
                    self.bv += rhs.bv;
                }
            }

            impl Sub for $nam {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self {
                    Self::new(self.s - rhs.s, self.bv - rhs.bv)
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.s -= rhs.s;
                    self.bv -= rhs.bv;
                }
            }

            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: Self) -> Self {
                    Self {
                        s: self.s * rhs.s - (self.bv.xy * rhs.bv.xy),
                        bv: $bv {
                            xy: self.s * rhs.bv.xy + rhs.s * self.bv.xy,
                        }
                    }
                }
            }

            impl Mul<$v2> for $nam {
                type Output = $v2;

                #[inline]
                fn mul(self, mut rhs: $v2) -> $v2 {
                    self.rotate_vec(&mut rhs);
                    rhs
                }
            }

            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.s *= rhs;
                    self.bv *= rhs;
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    Self::new(self.s * rhs, self.bv * rhs)
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rotor: $nam) -> $nam {
                    rotor * self
                }
            }

            impl Div<$t> for $nam {
                type Output = Self;
                #[inline]
                fn div(self, rhs: $t) -> Self {
                    Self::new(self.s / rhs, self.bv / rhs)
                }
            }

            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.s /= rhs;
                    self.bv /= rhs;
                }
            }
        )+
    };
}

impl_rotor2![(f32, Rot2, Bivec2, Vec2, Mat2)];
