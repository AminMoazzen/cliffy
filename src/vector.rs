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

    fn project(&mut self, other: Self);

    fn projected(&self, other: Self) -> Self;

    fn reject(&mut self, other: Self);

    fn rejected(&self, other: Self) -> Self;

    fn reflect(&mut self, other: Self);

    fn reflected(&self, other: Self) -> Self;

    fn reflect_normal(&mut self, normal: Self);

    fn reflected_normal(&self, normal: Self) -> Self;

    fn to_target(&self, target: Self) -> Self;

    fn dist_to_target(&self, target: Self) -> Self::Decimal;
}
