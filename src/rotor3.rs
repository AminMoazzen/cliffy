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
                pub fn from_angle_plane(angle: $t, plane: $bv) -> Self {
                    let half_angle = angle / 2.0;
                    let (sin, cos) = half_angle.sin_cos();
                    Self::new(cos, -sin * plane)
                }

                #[inline]
                pub fn rotate_vec(&self, vec: &mut $v3) {
                    let fx = self.s * vec.x + self.bv.xy * vec.y + self.bv.xz * vec.z;
                    let fy = self.s * vec.y - self.bv.xy * vec.x + self.bv.yz * vec.z;
                    let fz = self.s * vec.z - self.bv.xz * vec.x - self.bv.yz * vec.y;
                    let fw = self.bv.xy * vec.z - self.bv.xz * vec.y + self.bv.yz * vec.x;

                    vec.x = self.s * fx + self.bv.xy * fy + self.bv.xz * fz + self.bv.yz * fw;
                    vec.y = self.s * fy - self.bv.xy * fx - self.bv.xz * fw + self.bv.yz * fz;
                    vec.z = self.s * fz + self.bv.xy * fw - self.bv.xz * fx - self.bv.yz * fy;
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
                        s: self.s * rhs.s - self.bv.xy * rhs.bv.xy - self.bv.xz * rhs.bv.xz - self.bv.yz * rhs.bv.yz,
                        bv: $bv {
                            xy: self.bv.xy * rhs.s + self.s * rhs.bv.xy + self.bv.yz * rhs.bv.xz - self.bv.xz * rhs.bv.yz,
                            xz: self.bv.xz * rhs.s + self.s * rhs.bv.xz - self.bv.yz * rhs.bv.xy + self.bv.xy * rhs.bv.yz,
                            yz: self.bv.yz * rhs.s + self.s * rhs.bv.yz + self.bv.xz * rhs.bv.xy - self.bv.xy * rhs.bv.xz,
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
