use std::ops::*;

macro_rules! impl_bivec3 {
    [$(($t:ident, $nam:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub xy: $t,
                pub xz: $t,
                pub yz: $t,
            }

            impl $nam {
                pub fn new(xy: $t, xz: $t, yz: $t) -> Self {
                    Self {xy, xz, yz}
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::new(0.0, 0.0, 0.0)
                }
            }

            impl Add for $nam {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $nam) -> Self {
                    $nam::new(self.xy + rhs.xy, self.xz + rhs.xz, self.yz + rhs.yz)
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: $nam) {
                    self.xy += rhs.xy;
                    self.xz += rhs.xz;
                    self.yz += rhs.yz;
                }
            }

            impl Sub for $nam {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $nam) -> Self {
                    $nam::new(self.xy - rhs.xy, self.xz - rhs.xz, self.yz - rhs.yz)
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: $nam) {
                    self.xy -= rhs.xy;
                    self.xz -= rhs.xz;
                    self.yz -= rhs.yz;
                }
            }

            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $nam) -> Self {
                    $nam::new(self.xy * rhs.xy, self.xz * rhs.xz, self.yz * rhs.yz)
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> $nam {
                    $nam::new(self * rhs.xy, self * rhs.xz, self * rhs.yz)
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    $nam::new(self.xy * rhs, self.xz * rhs, self.yz * rhs)
                }
            }

            impl MulAssign for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: Self) {
                    self.xy *= rhs.xy;
                    self.xz *= rhs.xz;
                    self.yz *= rhs.yz;
                }
            }

            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.xy *= rhs;
                    self.xz *= rhs;
                    self.yz *= rhs;
                }
            }

            impl Div for $nam {
                type Output = Self;

                #[inline]
                fn div(self, rhs: $nam) -> Self {
                    $nam::new(self.xy / rhs.xy, self.xz / rhs.xz, self.yz / rhs.yz)
                }
            }

            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> $nam {
                    $nam::new(self.xy / rhs, self.xz / rhs, self.yz / rhs)
                }
            }

            impl DivAssign for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $nam) {
                    self.xy /= rhs.xy;
                    self.xz /= rhs.xz;
                    self.yz /= rhs.yz;
                }
            }

            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.xy /= rhs;
                    self.xz /= rhs;
                    self.yz /= rhs;
                }
            }

            impl Neg for $nam {
                type Output = Self;

                #[inline]
                fn neg(mut self) -> Self {
                    self.xy = -self.xy;
                    self.xz = -self.xz;
                    self.yz = -self.yz;
                    self
                }
            }
        )+
    };
}

impl_bivec3![(f32, Bivec3)];