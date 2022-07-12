pub trait Bivector {
    type Decimal;

    fn mag(&self) -> Self::Decimal;

    fn mag_sq(&self) -> Self::Decimal;

    fn dot(&self, other: Self) -> Self::Decimal;

    fn normalize(&mut self);

    fn normalized(&self) -> Self;
}
