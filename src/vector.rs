pub trait Vector {
    type Decimal;
    type Bivec;
    type Rotor;

    fn mag(&self) -> Self::Decimal;

    fn mag_sq(&self) -> Self::Decimal;

    fn dot(&self, other: Self) -> Self::Decimal;

    fn wedge(&self, other: Self) -> Self::Bivec;

    fn geom(&self, other: Self) -> Self::Rotor;

    fn normalize(&mut self);

    fn normalized(&self) -> Self;

    fn inverse(&self) -> Self;

    fn reflect(&self, other: Self) -> Self;

    fn reject(&self, other: Self) -> Self;

    fn project(&self, other: Self) -> Self;

    fn to_target(&self, target: Self) -> Self;

    fn dist_to_target(&self, target: Self) -> Self::Decimal;
}
