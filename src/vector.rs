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

    fn to_other(&self, other: Self) -> Self;

    fn dist_to_other(&self, other: Self) -> Self::Decimal;

    fn angle_between(&self, other: Self) -> Self::Decimal;

    fn angle_between_normal(&self, normal: Self) -> Self::Decimal;

    fn lerp(&self, to: Self, to: Self::Decimal) -> Self;

    fn slerp(&self, to: Self, to: Self::Decimal) -> Self;

    fn nlerp(&self, to: Self, to: Self::Decimal) -> Self;
}
