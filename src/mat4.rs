use crate::*;
use std::ops::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

macro_rules! impl_mat4 {
    [$(($t:ident, $nam:ident, $col:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub cols: [$col; 4],
            }

            impl $nam {
                #[inline]
                pub fn new(col0: $col, col1: $col, col2: $col, col3: $col) -> Self {
                    Self { cols: [col0, col1, col2, col3] }
                }

                #[inline]
                pub fn identity() -> Self {
                    Self::new(
                        $col::new(1.0, 0.0, 0.0, 0.0),
                        $col::new(0.0, 1.0, 0.0, 0.0),
                        $col::new(0.0, 0.0, 1.0, 0.0),
                        $col::new(0.0, 0.0, 0.0, 1.0),
                    )
                }

                #[inline]
                pub fn zero() -> Self {
                    Self::new($col::zero(), $col::zero(), $col::zero(), $col::zero())
                }

                #[inline]
                pub fn transpose(&self) -> Self {
                    let (c0, c1, c2, c3) = (self.cols[0], self.cols[1], self.cols[2], self.cols[3]);
                    Self::new(
                        $col::new(c0.x, c1.x, c2.x, c3.x),
                        $col::new(c0.y, c1.y, c2.y, c3.y),
                        $col::new(c0.z, c1.z, c2.z, c3.z),
                        $col::new(c0.w, c1.w, c2.w, c3.w),
                    )
                }

                #[inline]
                pub fn det(&self) -> $t {
                    let (c0, c1, c2, c3) = (self.cols[0], self.cols[1], self.cols[2], self.cols[3]);

                    let s0 = c0.x * c1.y - c1.x * c0.y;
                    let s1 = c0.x * c2.y - c2.x * c0.y;
                    let s2 = c0.x * c3.y - c3.x * c0.y;
                    let s3 = c1.x * c2.y - c2.x * c1.y;
                    let s4 = c1.x * c3.y - c3.x * c1.y;
                    let s5 = c2.x * c3.y - c3.x * c2.y;

                    let c5 = c2.z * c3.w - c3.z * c2.w;
                    let c4 = c1.z * c3.w - c3.z * c1.w;
                    let c3_ = c1.z * c2.w - c2.z * c1.w;
                    let c2_ = c0.z * c3.w - c3.z * c0.w;
                    let c1_ = c0.z * c2.w - c2.z * c0.w;
                    let c0_ = c0.z * c1.w - c1.z * c0.w;

                    s0 * c5 - s1 * c4 + s2 * c3_ + s3 * c2_ - s4 * c1_ + s5 * c0_
                }

                #[inline]
                pub fn inverse(&self) -> Option<Self> {
                    let (c0, c1, c2, c3) = (self.cols[0], self.cols[1], self.cols[2], self.cols[3]);

                    let s0 = c0.x * c1.y - c1.x * c0.y;
                    let s1 = c0.x * c2.y - c2.x * c0.y;
                    let s2 = c0.x * c3.y - c3.x * c0.y;
                    let s3 = c1.x * c2.y - c2.x * c1.y;
                    let s4 = c1.x * c3.y - c3.x * c1.y;
                    let s5 = c2.x * c3.y - c3.x * c2.y;

                    let c5 = c2.z * c3.w - c3.z * c2.w;
                    let c4 = c1.z * c3.w - c3.z * c1.w;
                    let c3_ = c1.z * c2.w - c2.z * c1.w;
                    let c2_ = c0.z * c3.w - c3.z * c0.w;
                    let c1_ = c0.z * c2.w - c2.z * c0.w;
                    let c0_ = c0.z * c1.w - c1.z * c0.w;

                    let det = s0 * c5 - s1 * c4 + s2 * c3_ + s3 * c2_ - s4 * c1_ + s5 * c0_;
                    if det == 0.0 {
                        return None;
                    }
                    let inv_det = 1.0 / det;

                    Some(Self::new(
                        $col::new(
                            ( c1.y * c5 - c2.y * c4 + c3.y * c3_) * inv_det,
                            (-c0.y * c5 + c2.y * c2_ - c3.y * c1_) * inv_det,
                            ( c0.y * c4 - c1.y * c2_ + c3.y * c0_) * inv_det,
                            (-c0.y * c3_ + c1.y * c1_ - c2.y * c0_) * inv_det,
                        ),
                        $col::new(
                            (-c1.x * c5 + c2.x * c4 - c3.x * c3_) * inv_det,
                            ( c0.x * c5 - c2.x * c2_ + c3.x * c1_) * inv_det,
                            (-c0.x * c4 + c1.x * c2_ - c3.x * c0_) * inv_det,
                            ( c0.x * c3_ - c1.x * c1_ + c2.x * c0_) * inv_det,
                        ),
                        $col::new(
                            ( c1.w * s5 - c2.w * s4 + c3.w * s3) * inv_det,
                            (-c0.w * s5 + c2.w * s2 - c3.w * s1) * inv_det,
                            ( c0.w * s4 - c1.w * s2 + c3.w * s0) * inv_det,
                            (-c0.w * s3 + c1.w * s1 - c2.w * s0) * inv_det,
                        ),
                        $col::new(
                            (-c1.z * s5 + c2.z * s4 - c3.z * s3) * inv_det,
                            ( c0.z * s5 - c2.z * s2 + c3.z * s1) * inv_det,
                            (-c0.z * s4 + c1.z * s2 - c3.z * s0) * inv_det,
                            ( c0.z * s3 - c1.z * s1 + c2.z * s0) * inv_det,
                        ),
                    ))
                }
            }

            // ── Mat * Vec — SSE2 path ─────────────────────────────────────────
            // Broadcast each component of rhs across a full __m128 lane via
            // _mm_shuffle_ps, then multiply by the corresponding column and
            // accumulate. This turns 16 scalar muls + 12 adds into 4 SIMD muls
            // + 3 SIMD adds, computing all four output components in parallel.
            #[cfg(target_arch = "x86_64")]
            impl Mul<$col> for $nam {
                type Output = $col;

                #[inline]
                fn mul(self, rhs: $col) -> $col {
                    unsafe {
                        let col0 = _mm_loadu_ps(&self.cols[0].x as *const $t as *const f32);
                        let col1 = _mm_loadu_ps(&self.cols[1].x as *const $t as *const f32);
                        let col2 = _mm_loadu_ps(&self.cols[2].x as *const $t as *const f32);
                        let col3 = _mm_loadu_ps(&self.cols[3].x as *const $t as *const f32);
                        let v    = _mm_loadu_ps(&rhs.x as *const $t as *const f32);

                        // Shuffle imm8 = (w<<6)|(z<<4)|(y<<2)|x selects from src1/src2.
                        // With both args identical we get a full broadcast of one lane.
                        // 0x00 = 0b00_00_00_00 → [v.x, v.x, v.x, v.x]
                        // 0x55 = 0b01_01_01_01 → [v.y, v.y, v.y, v.y]
                        // 0xAA = 0b10_10_10_10 → [v.z, v.z, v.z, v.z]
                        // 0xFF = 0b11_11_11_11 → [v.w, v.w, v.w, v.w]
                        let vx = _mm_shuffle_ps(v, v, 0x00);
                        let vy = _mm_shuffle_ps(v, v, 0x55);
                        let vz = _mm_shuffle_ps(v, v, 0xAA);
                        let vw = _mm_shuffle_ps(v, v, 0xFF);

                        let r = _mm_add_ps(
                            _mm_add_ps(_mm_mul_ps(col0, vx), _mm_mul_ps(col1, vy)),
                            _mm_add_ps(_mm_mul_ps(col2, vz), _mm_mul_ps(col3, vw)),
                        );

                        let mut out = std::mem::MaybeUninit::<$col>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            // ── Mat * Vec — scalar fallback ───────────────────────────────────
            #[cfg(not(target_arch = "x86_64"))]
            impl Mul<$col> for $nam {
                type Output = $col;

                #[inline]
                fn mul(self, rhs: $col) -> $col {
                    $col::new(
                        self.cols[0].x * rhs.x + self.cols[1].x * rhs.y + self.cols[2].x * rhs.z + self.cols[3].x * rhs.w,
                        self.cols[0].y * rhs.x + self.cols[1].y * rhs.y + self.cols[2].y * rhs.z + self.cols[3].y * rhs.w,
                        self.cols[0].z * rhs.x + self.cols[1].z * rhs.y + self.cols[2].z * rhs.z + self.cols[3].z * rhs.w,
                        self.cols[0].w * rhs.x + self.cols[1].w * rhs.y + self.cols[2].w * rhs.z + self.cols[3].w * rhs.w,
                    )
                }
            }

            // Mat * Mat delegates to Mat * Vec, inheriting the SIMD path above.
            impl Mul for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: Self) -> Self {
                    Self::new(
                        self * rhs.cols[0],
                        self * rhs.cols[1],
                        self * rhs.cols[2],
                        self * rhs.cols[3],
                    )
                }
            }

            impl Mul<$t> for $nam {
                type Output = Self;

                #[inline]
                fn mul(self, rhs: $t) -> Self {
                    Self::new(
                        self.cols[0] * rhs,
                        self.cols[1] * rhs,
                        self.cols[2] * rhs,
                        self.cols[3] * rhs,
                    )
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
                        self.cols[3] + rhs.cols[3],
                    )
                }
            }

            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.cols[0] += rhs.cols[0];
                    self.cols[1] += rhs.cols[1];
                    self.cols[2] += rhs.cols[2];
                    self.cols[3] += rhs.cols[3];
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
                        self.cols[3] - rhs.cols[3],
                    )
                }
            }

            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.cols[0] -= rhs.cols[0];
                    self.cols[1] -= rhs.cols[1];
                    self.cols[2] -= rhs.cols[2];
                    self.cols[3] -= rhs.cols[3];
                }
            }

            impl Neg for $nam {
                type Output = Self;

                #[inline]
                fn neg(self) -> Self {
                    Self::new(-self.cols[0], -self.cols[1], -self.cols[2], -self.cols[3])
                }
            }

            impl Index<usize> for $nam {
                type Output = $col;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.cols[0],
                        1 => &self.cols[1],
                        2 => &self.cols[2],
                        3 => &self.cols[3],
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
                        3 => &mut self.cols[3],
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }
        )+
    };
}

impl_mat4![(f32, Mat4, Vec4)];
