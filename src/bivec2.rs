use std::ops::*;

macro_rules! impl_bivec2 {
    [$(($t:ident, $nam:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub xy: $t
            }

            impl $nam {
                pub fn new(xy: $t) -> Self {
                    Self {xy}
                }
            }

            impl Add for $nam {
                type Output = Self;

                #[inline]
                fn add(self, rhs: $nam) -> Self {
                    $nam::new(self.xy + rhs.xy)
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: $nam) {
                    self.xy += rhs.xy;
                }
            }

            impl Sub for $nam {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: $nam) -> Self {
                    $nam::new(self.xy - rhs.xy)
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: $nam) {
                    self.xy -= rhs.xy;
                }
            }

            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $nam) -> Self {
                    $nam::new(self.xy * rhs.xy)
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> $nam {
                    $nam::new(self * rhs.xy)
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    $nam::new(self.xy * rhs)
                }
            }

            impl MulAssign for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: Self) {
                    self.xy *= rhs.xy;
                }
            }

            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.xy *= rhs;
                }
            }

            impl Div for $nam {
                type Output = Self;

                #[inline]
                fn div(self, rhs: $nam) -> Self {
                    $nam::new(self.xy / rhs.xy)
                }
            }

            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> $nam {
                    $nam::new(self.xy / rhs)
                }
            }

            impl DivAssign for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $nam) {
                    self.xy /= rhs.xy;
                }
            }

            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.xy /= rhs;
                }
            }

            impl Neg for $nam {
                type Output = Self;

                #[inline]
                fn neg(mut self) -> Self {
                    self.xy = -self.xy;
                    self
                }
            }
        )+
    };
}

impl_bivec2![(f32, Bivec2)];
