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
            }

            impl Rotor for $nam {
                type Decimal = $t;
                type Vector = $v3;
                type Matrix = $m3;

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
                    self.bv.xz /= mag;
                    self.bv.yz /= mag;
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
                    let sa2 = a.s * a.s;
                    let baxy2 = a.bv.xy * a.bv.xy;
                    let baxz2 = a.bv.xz * a.bv.xz;
                    let bayz2 = a.bv.yz * a.bv.yz;
                    let sa_baxy = a.s * a.bv.xy;
                    let sa_baxz = a.s * a.bv.xz;
                    let sa_bayz = a.s * a.bv.yz;
                    let baxy_baxz = a.bv.xy * a.bv.xz;
                    let baxy_bayz = a.bv.xy * a.bv.yz;
                    let baxz_bayz = a.bv.xz * a.bv.yz;
                    let two_bbxy =  2.0 * b.bv.xy;
                    let two_bbxz =  2.0 * b.bv.xz;
                    let two_bbyz =  2.0 * b.bv.yz;

                    self.s = (sa2 + baxy2 + baxz2 + bayz2) * b.s;

                    self.bv.xy = (sa2 + baxy2 - baxz2 - bayz2) * b.bv.xy
                        + (baxy_baxz + sa_bayz) * two_bbxz
                        + (baxy_bayz - sa_baxz) * two_bbyz;

                    self.bv.xz = (sa2 - baxy2 + baxz2 - bayz2) * b.bv.xz
                        + (baxy_baxz - sa_bayz) * two_bbxy
                        + (baxz_bayz + sa_baxy) * two_bbyz;

                    self.bv.yz = (sa2 - baxy2 - baxz2 + bayz2) * b.bv.yz
                        + (baxy_bayz + sa_baxz) * two_bbxy
                        + (baxz_bayz - sa_baxy) * two_bbxz;
                }

                #[inline]
                fn rotate_vec(&self, vec: &mut $v3) {
                    let fx = self.s * vec.x + self.bv.xy * vec.y + self.bv.xz * vec.z;
                    let fy = self.s * vec.y - self.bv.xy * vec.x + self.bv.yz * vec.z;
                    let fz = self.s * vec.z - self.bv.xz * vec.x - self.bv.yz * vec.y;
                    let fw = self.bv.xy * vec.z - self.bv.xz * vec.y + self.bv.yz * vec.x;

                    vec.x = self.s * fx + self.bv.xy * fy + self.bv.xz * fz + self.bv.yz * fw;
                    vec.y = self.s * fy - self.bv.xy * fx - self.bv.xz * fw + self.bv.yz * fz;
                    vec.z = self.s * fz + self.bv.xy * fw - self.bv.xz * fx - self.bv.yz * fy;
                }

                #[inline]
                fn rotated_by(&self, other: Self) -> Self {
                    let mut r = self.clone();
                    r.rotate_by(other);
                    r
                }

                #[inline]
                fn into_matrix(&self) -> Self::Matrix {
                    let s2 = self.s * self.s;
                    let bxy2 = self.bv.xy * self.bv.xy;
                    let bxz2 = self.bv.xz * self.bv.xz;
                    let byz2 = self.bv.yz * self.bv.yz;
                    let s_bxy = self.s * self.bv.xy;
                    let s_bxz = self.s * self.bv.xz;
                    let s_byz = self.s * self.bv.yz;
                    let bxz_byz = self.bv.xz * self.bv.yz;
                    let bxy_byz = self.bv.xy * self.bv.yz;
                    let bxy_bxz = self.bv.xy * self.bv.xz;

                    $m3::new(
                        $v3::new(
                            s2 - bxy2 - bxz2 + byz2,
                            -2.0 * (bxz_byz + s_bxy),
                            2.0 * (bxy_byz - s_bxz)),
                        $v3::new(
                            2.0 * (s_bxy - bxz_byz),
                            s2 - bxy2 + bxz2 - byz2,
                            -2.0 * (s_byz + bxy_bxz)
                        ),
                        $v3::new(
                            2.0 * (s_bxz + bxy_byz),
                            2.0 * (s_byz - bxy_bxz),
                            s2 + bxy2 - bxz2 - byz2
                        )
                    )
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

impl_rotor3![(f32, Rot3, Bivec3, Vec3, Mat3)];
