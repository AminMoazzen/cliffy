pub(crate) trait GenValue<T> {
    fn gen(val: T) -> Self;
}

impl GenValue<f32> for f32 {
    #[inline(always)]
    fn gen(val: f32) -> Self {
        val
    }
}

impl GenValue<f64> for f64 {
    #[inline(always)]
    fn gen(val: f64) -> Self {
        val
    }
}

impl GenValue<i32> for i32 {
    #[inline(always)]
    fn gen(val: i32) -> Self {
        val
    }
}
