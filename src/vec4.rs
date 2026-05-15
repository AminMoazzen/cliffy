use crate::*;
use std::ops::*;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

// SSE2-only horizontal sum of all four f32 lanes.
// Used as the dot-product fallback when SSE4.1 is absent.
#[cfg(all(target_arch = "x86_64", not(target_feature = "sse4.1")))]
#[inline(always)]
unsafe fn hsum_ps_sse2(v: __m128) -> f32 {
    // [x,y,z,w] → swap pairs → [y,x,w,z]; add → [x+y, x+y, z+w, z+w]
    let shuf = _mm_shuffle_ps(v, v, 0xB1);
    let sums = _mm_add_ps(v, shuf);
    // move high 64-bits to low → [z+w, z+w, …]; add lane 0 → x+y+z+w
    let shuf = _mm_movehl_ps(shuf, sums);
    _mm_cvtss_f32(_mm_add_ss(sums, shuf))
}

macro_rules! impl_vec4 {
    [$(($t:ident, $nam:ident, $v2:ident, $v3:ident)), +] => {
        $(
            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            #[repr(C)]
            pub struct $nam {
                pub x: $t,
                pub y: $t,
                pub z: $t,
                pub w: $t,
            }

            // ── Common methods ────────────────────────────────────────────────
            impl $nam {
                #[inline]
                pub fn new(x: $t, y: $t, z: $t, w: $t) -> Self {
                    Self { x, y, z, w }
                }

                #[inline]
                pub fn uni(val: $t) -> Self {
                    Self::new(val, val, val, val)
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
                pub fn mag(&self) -> $t {
                    self.mag_sq().sqrt()
                }

                #[inline]
                pub fn normalize(&mut self) {
                    let mag = self.mag();
                    *self /= mag;
                }

                #[inline]
                pub fn normalized(&self) -> Self {
                    let mut v = self.clone();
                    v.normalize();
                    v
                }

                #[inline]
                pub fn project(&mut self, other: Self) {
                    *self = (self.dot(other) / other.mag_sq()) * other;
                }

                #[inline]
                pub fn projected(&self, other: Self) -> Self {
                    let mut v = self.clone();
                    v. project(other);
                    v
                }

                #[inline]
                pub fn reject(&mut self, other: Self) {
                    *self -= (self.dot(other) / other.mag_sq()) * other;
                }

                #[inline]
                pub fn rejected(&self, other: Self) -> Self {
                    let mut v = self.clone();
                    v. reject(other);
                    v
                }

                #[inline]
                pub fn reflect(&mut self, other: Self) {
                    *self -= 2.0 * (self.dot(other) / other.mag_sq()) * other;
                }

                #[inline]
                pub fn reflected(&self, other: Self) -> Self {
                    let mut v = self.clone();
                    v. reflect(other);
                    v
                }

                #[inline]
                pub fn reflect_normal(&mut self, normal: Self) {
                    *self -= 2.0 * self.dot(normal) * normal;
                }

                #[inline]
                pub fn reflected_normal(&self, normal: Self) -> Self {
                    let mut v = self.clone();
                    v. reflect_normal(normal);
                    v
                }

                #[inline]
                pub fn to(&self, other: Self) -> Self {
                    other - *self
                }

                #[inline]
                pub fn distance(&self, other: Self) -> $t {
                    self.to(other).mag()
                }

                #[inline]
                pub fn lerp(&self, to: Self, t: $t) -> Self {
                    (1.0 - t) * *self + t * to
                }

                #[inline]
                pub fn nlerp(&self, to: Self, t: $t) -> Self {
                    self.lerp(to, t).normalized()
                }
            }

            // ── SIMD dot / mag_sq — SSE4.1 path ──────────────────────────────
            // dpps computes the dot product of all four lanes in one instruction.
            // 0xF1 = use all 4 source lanes, write result to lane 0 only.
            #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
            impl $nam {
                #[inline]
                pub fn mag_sq(&self) -> $t {
                    unsafe {
                        let v = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        _mm_cvtss_f32(_mm_dp_ps(v, v, 0xF1))
                    }
                }

                #[inline]
                pub fn dot(&self, other: Self) -> $t {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&other.x as *const $t as *const f32);
                        _mm_cvtss_f32(_mm_dp_ps(a, b, 0xF1))
                    }
                }
            }

            // ── SIMD dot / mag_sq — SSE2 path (no SSE4.1) ────────────────────
            #[cfg(all(target_arch = "x86_64", not(target_feature = "sse4.1")))]
            impl $nam {
                #[inline]
                pub fn mag_sq(&self) -> $t {
                    unsafe {
                        let v = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        hsum_ps_sse2(_mm_mul_ps(v, v))
                    }
                }

                #[inline]
                pub fn dot(&self, other: Self) -> $t {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&other.x as *const $t as *const f32);
                        hsum_ps_sse2(_mm_mul_ps(a, b))
                    }
                }
            }

            // ── Scalar dot / mag_sq — non-x86 fallback ───────────────────────
            #[cfg(not(target_arch = "x86_64"))]
            impl $nam {
                #[inline]
                pub fn mag_sq(&self) -> $t {
                    (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
                }

                #[inline]
                pub fn dot(&self, other: Self) -> $t {
                    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
                }
            }

            // ── Add ───────────────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl Add for $nam {
                type Output = $nam;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&rhs.x as *const $t as *const f32);
                        let r = _mm_add_ps(a, b);
                        let mut out = std::mem::MaybeUninit::<Self>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Add for $nam {
                type Output = $nam;

                #[inline]
                fn add(self, rhs: Self) -> Self::Output {
                    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
                }
            }

            // ── AddAssign ─────────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&rhs.x as *const $t as *const f32);
                        _mm_storeu_ps(&mut self.x as *mut $t as *mut f32, _mm_add_ps(a, b));
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl AddAssign for $nam {
                #[inline]
                fn add_assign(&mut self, rhs: Self) {
                    self.x += rhs.x;
                    self.y += rhs.y;
                    self.z += rhs.z;
                    self.w += rhs.w;
                }
            }

            // ── Sub ───────────────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl Sub for $nam {
                type Output = $nam;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&rhs.x as *const $t as *const f32);
                        let r = _mm_sub_ps(a, b);
                        let mut out = std::mem::MaybeUninit::<Self>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Sub for $nam {
                type Output = $nam;

                #[inline]
                fn sub(self, rhs: Self) -> Self::Output {
                    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w - rhs.w)
                }
            }

            // ── SubAssign ─────────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_loadu_ps(&rhs.x as *const $t as *const f32);
                        _mm_storeu_ps(&mut self.x as *mut $t as *mut f32, _mm_sub_ps(a, b));
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl SubAssign for $nam {
                #[inline]
                fn sub_assign(&mut self, rhs: Self) {
                    self.x -= rhs.x;
                    self.y -= rhs.y;
                    self.z -= rhs.z;
                    self.w -= rhs.w;
                }
            }

            // ── Neg ───────────────────────────────────────────────────────────
            // XOR with the IEEE 754 sign bit flips the sign of every lane.
            #[cfg(target_arch = "x86_64")]
            impl Neg for $nam {
                type Output = $nam;

                #[inline]
                fn neg(self) -> $nam {
                    unsafe {
                        let v = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let sign_mask = _mm_set1_ps(-0.0f32);
                        let r = _mm_xor_ps(v, sign_mask);
                        let mut out = std::mem::MaybeUninit::<Self>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Neg for $nam {
                type Output = $nam;

                #[inline]
                fn neg(self) -> $nam {
                    self * -1.0
                }
            }

            // ── Mul<scalar> ───────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl Mul<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $t) -> Self::Output {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_set1_ps(rhs as f32);
                        let r = _mm_mul_ps(a, b);
                        let mut out = std::mem::MaybeUninit::<Self>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Mul<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $t) -> Self::Output {
                    Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
                }
            }

            // ── MulAssign<scalar> ─────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_set1_ps(rhs as f32);
                        _mm_storeu_ps(&mut self.x as *mut $t as *mut f32, _mm_mul_ps(a, b));
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl MulAssign<$t> for $nam {
                #[inline]
                fn mul_assign(&mut self, rhs: $t) {
                    self.x *= rhs;
                    self.y *= rhs;
                    self.z *= rhs;
                    self.w *= rhs;
                }
            }

            // scalar * vec: reuse vec * scalar
            impl Mul<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn mul(self, rhs: $nam) -> Self::Output {
                    rhs * self
                }
            }

            // ── Div<scalar> ───────────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> Self::Output {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_set1_ps(rhs as f32);
                        let r = _mm_div_ps(a, b);
                        let mut out = std::mem::MaybeUninit::<Self>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Div<$t> for $nam {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $t) -> Self::Output {
                    Self::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
                }
            }

            // ── DivAssign<scalar> ─────────────────────────────────────────────
            #[cfg(target_arch = "x86_64")]
            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    unsafe {
                        let a = _mm_loadu_ps(&self.x as *const $t as *const f32);
                        let b = _mm_set1_ps(rhs as f32);
                        _mm_storeu_ps(&mut self.x as *mut $t as *mut f32, _mm_div_ps(a, b));
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl DivAssign<$t> for $nam {
                #[inline]
                fn div_assign(&mut self, rhs: $t) {
                    self.x /= rhs;
                    self.y /= rhs;
                    self.z /= rhs;
                    self.w /= rhs;
                }
            }

            // ── scalar / vec (component-wise reciprocal) ──────────────────────
            #[cfg(target_arch = "x86_64")]
            impl Div<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $nam) -> Self::Output {
                    unsafe {
                        let a = _mm_set1_ps(self as f32);
                        let b = _mm_loadu_ps(&rhs.x as *const $t as *const f32);
                        let r = _mm_div_ps(a, b);
                        let mut out = std::mem::MaybeUninit::<$nam>::uninit();
                        _mm_storeu_ps(out.as_mut_ptr() as *mut f32, r);
                        out.assume_init()
                    }
                }
            }

            #[cfg(not(target_arch = "x86_64"))]
            impl Div<$nam> for $t {
                type Output = $nam;

                #[inline]
                fn div(self, rhs: $nam) -> Self::Output {
                    $nam::new(self / rhs.x, self / rhs.y, self / rhs.z, self / rhs.w)
                }
            }

            // ── Indexing, conversions (unchanged) ─────────────────────────────
            impl Index<usize> for $nam {
                type Output =  $t;

                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        2 => &self.z,
                        3 => &self.w,
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
                        3 => &mut self.w,
                        _i => panic!("{} is not a valid index for {}", _i, std::any::type_name::<$nam>()),
                    }
                }
            }

            impl Into<[$t; 4]> for $nam {
                #[inline]
                fn into(self) -> [$t; 4] {
                    [self.x, self.y, self.z, self.w]
                }
            }

            impl From<[$t; 4]> for $nam {
                #[inline]
                fn from(comps: [$t; 4]) -> Self {
                    Self::new(comps[0], comps[1], comps[2], comps[3])
                }
            }

            impl From<&[$t; 4]> for $nam {
                #[inline]
                fn from(comps: &[$t; 4]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<&mut [$t; 4]> for $nam {
                #[inline]
                fn from(comps: &mut [$t; 4]) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<($t, $t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: ($t, $t, $t, $t)) -> Self {
                    Self::new(comps.0, comps.1, comps.2, comps.3)
                }
            }

            impl From<&($t, $t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: &($t, $t, $t, $t)) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<&mut ($t, $t, $t, $t)> for $nam {
                #[inline]
                fn from(comps: &mut ($t, $t, $t, $t)) -> Self {
                    Self::from(*comps)
                }
            }

            impl From<$nam> for ($t, $t) {
                #[inline]
                fn from(v: $nam) -> Self {
                    (v.x, v.y)
                }
            }

            impl From<$v2> for $nam {
                #[inline]
                fn from(vec2: $v2) -> Self {
                    Self::new(vec2.x, vec2.y, 0.0, 0.0)
                }
            }

            impl From<$v3> for $nam {
                #[inline]
                fn from(vec4: $v3) -> Self {
                    Self::new(vec4.x, vec4.y, vec4.z, 0.0)
                }
            }
        )+
    };
}

impl_vec4![(f32, Vec4, Vec2, Vec3)];
