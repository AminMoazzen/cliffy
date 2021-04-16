use crate::*;
use std::ops::*;

macro_rules! impl_rotor3 {
    [$(($t:ident, $nam:ident, $bv:ident, $v3:ident, $m3:ident)), +] => {
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

                #[inline]
                pub fn rotate_vec(&self, vec: &mut $v3) {
                    let fx = self.s * vec.x + self.bv.xy * vec.y;
                    let fy = self.s * vec.y - (self.bv.xy * vec.x);

                    vec.x = self.s * fx + self.bv.xy * fy;
                    vec.y = self.s * fy - (self.bv.xy * fx);
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

            impl Mul<$v3> for $nam {
                type Output = $v3;

                #[inline]
                fn mul(self, mut rhs: $v3) -> $v3 {
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

impl_rotor3![(f32, Rotor3, Bivec3, Vec3, Mat3)];
