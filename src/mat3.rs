use crate::*;
use std::ops::*;

macro_rules! impl_mat3 {
    [$(($t:ident, $nam:ident, $col:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub cols: [$col; 3],
            }

            impl $nam {
                #[inline]
                pub fn new(col0: $col, col1: $col, col2: $col) -> Self {
                    Self { cols: [col0, col1, col2] }
                }

                #[inline]
                pub fn identity() -> Self {
                    Self::new(
                        $col::new(1.0, 0.0, 0.0),
                        $col::new(0.0, 1.0, 0.0),
                        $col::new(0.0, 0.0, 1.0),
                    )
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::new($col::zero(), $col::zero(), $col::zero())
                }

                #[inline]
                pub fn transpose(&self) -> Self {
                    let (c0, c1, c2) = (self.cols[0], self.cols[1], self.cols[2]);
                    Self::new(
                        $col::new(c0.x, c1.x, c2.x),
                        $col::new(c0.y, c1.y, c2.y),
                        $col::new(c0.z, c1.z, c2.z),
                    )
                }

                #[inline]
                pub fn det(&self) -> $t {
                    let (c0, c1, c2) = (self.cols[0], self.cols[1], self.cols[2]);
                    c0.x * (c1.y * c2.z - c2.y * c1.z)
                        - c1.x * (c0.y * c2.z - c2.y * c0.z)
                        + c2.x * (c0.y * c1.z - c1.y * c0.z)
                }

                #[inline]
                pub fn inverse(&self) -> Option<Self> {
                    let det = self.det();
                    if det == 0.0 {
                        return None;
                    }
                    let inv_det = 1.0 / det;
                    let (a, d, g) = (self.cols[0].x, self.cols[0].y, self.cols[0].z);
                    let (b, e, h) = (self.cols[1].x, self.cols[1].y, self.cols[1].z);
                    let (c, f, i) = (self.cols[2].x, self.cols[2].y, self.cols[2].z);
                    Some(Self::new(
                        $col::new(
                            (e * i - f * h) * inv_det,
                            (f * g - d * i) * inv_det,
                            (d * h - e * g) * inv_det,
                        ),
                        $col::new(
                            (c * h - b * i) * inv_det,
                            (a * i - c * g) * inv_det,
                            (b * g - a * h) * inv_det,
                        ),
                        $col::new(
                            (b * f - c * e) * inv_det,
                            (c * d - a * f) * inv_det,
                            (a * e - b * d) * inv_det,
                        ),
                    ))
                }
            }

            impl Mul<$col> for $nam {
                type Output = $col;

                #[inline]
                fn mul(self, rhs: $col) -> $col {
                    $col::new(
                        self.cols[0].x * rhs.x + self.cols[1].x * rhs.y + self.cols[2].x * rhs.z,
                        self.cols[0].y * rhs.x + self.cols[1].y * rhs.y + self.cols[2].y * rhs.z,
                        self.cols[0].z * rhs.x + self.cols[1].z * rhs.y + self.cols[2].z * rhs.z,
                    )
                }
            }

            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: Self) -> Self {
                    Self::new(self * rhs.cols[0], self * rhs.cols[1], self * rhs.cols[2])
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    Self::new(self.cols[0] * rhs, self.cols[1] * rhs, self.cols[2] * rhs)
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
                    Self::new(
                        self.cols[0] + rhs.cols[0],
                        self.cols[1] + rhs.cols[1],
                        self.cols[2] + rhs.cols[2],
                    )
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.cols[0] += rhs.cols[0];
                    self.cols[1] += rhs.cols[1];
                    self.cols[2] += rhs.cols[2];
                }
            }

            impl Sub for $nam {
                type Output = Self;

                #[inline]
                fn sub(self, rhs: Self) -> Self {
                    Self::new(
                        self.cols[0] - rhs.cols[0],
                        self.cols[1] - rhs.cols[1],
                        self.cols[2] - rhs.cols[2],
                    )
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.cols[0] -= rhs.cols[0];
                    self.cols[1] -= rhs.cols[1];
                    self.cols[2] -= rhs.cols[2];
                }
            }

            impl Neg for $nam {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self {
                    Self::new(-self.cols[0], -self.cols[1], -self.cols[2])
                }
            }

            impl Index<usize> for $nam {
                type Output = $col;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.cols[0],
                        1 => &self.cols[1],
                        2 => &self.cols[2],
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl IndexMut<usize> for $nam {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.cols[0],
                        1 => &mut self.cols[1],
                        2 => &mut self.cols[2],
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }
        )+
    };
}

impl_mat3![(f32, Mat3, Vec3)];
