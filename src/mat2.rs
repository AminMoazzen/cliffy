use crate::*;
use std::ops::*;

macro_rules! impl_mat2 {
    [$(($t:ident, $nam:ident, $col:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub cols: [$col; 2],
            }

            impl $nam {
                #[inline]
                pub fn new(col0: $col, col1: $col) -> Self {
                    Self { cols: [col0, col1] }
                }

                #[inline]
                pub fn identity() -> Self {
                    Self::new(
                        $col::new(1.0, 0.0),
                        $col::new(0.0, 1.0),
                    )
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::new($col::zero(), $col::zero())
                }

                #[inline]
                pub fn transpose(&self) -> Self {
                    Self::new(
                        $col::new(self.cols[0].x, self.cols[1].x),
                        $col::new(self.cols[0].y, self.cols[1].y),
                    )
                }

                #[inline]
                pub fn det(&self) -> $t {
                    self.cols[0].x * self.cols[1].y - self.cols[1].x * self.cols[0].y
                }

                #[inline]
                pub fn inverse(&self) -> Option<Self> {
                    let det = self.det();
                    if det == 0.0 {
                        return None;
                    }
                    let inv_det = 1.0 / det;
                    Some(Self::new(
                        $col::new( self.cols[1].y * inv_det, -self.cols[0].y * inv_det),
                        $col::new(-self.cols[1].x * inv_det,  self.cols[0].x * inv_det),
                    ))
                }
            }

            impl Mul<$col> for $nam {
                type Output = $col;

                #[inline]
                fn mul(self, rhs: $col) -> $col {
                    $col::new(
                        self.cols[0].x * rhs.x + self.cols[1].x * rhs.y,
                        self.cols[0].y * rhs.x + self.cols[1].y * rhs.y,
                    )
                }
            }

            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: Self) -> Self {
                    Self::new(self * rhs.cols[0], self * rhs.cols[1])
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    Self::new(self.cols[0] * rhs, self.cols[1] * rhs)
                }
            }

            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> $nam {
                    rhs * self
                }
            }

            impl Add for $nam {
                type Output = Self;

                #[inline]
                fn add(self, rhs: Self) -> Self {
                    Self::new(self.cols[0] + rhs.cols[0], self.cols[1] + rhs.cols[1])
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.cols[0] += rhs.cols[0];
                    self.cols[1] += rhs.cols[1];
                }
            }

            impl Sub for $nam {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self {
                    Self::new(self.cols[0] - rhs.cols[0], self.cols[1] - rhs.cols[1])
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.cols[0] -= rhs.cols[0];
                    self.cols[1] -= rhs.cols[1];
                }
            }

            impl Neg for $nam {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self {
                    Self::new(-self.cols[0], -self.cols[1])
                }
            }

            impl Index<usize> for $nam {
                type Output = $col;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.cols[0],
                        1 => &self.cols[1],
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl IndexMut<usize> for $nam {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.cols[0],
                        1 => &mut self.cols[1],
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }
        )+
    };
}

impl_mat2![(f32, Mat2, Vec2)];
