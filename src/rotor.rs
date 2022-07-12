pub trait Rotor {
    type Decimal;
    type Vector;
    type Matrix;

    fn mag(&self) -> Self::Decimal;

    fn mag_sq(&self) -> Self::Decimal;

    fn dot(&self, other: Self) -> Self::Decimal;

    fn normalize(&mut self);

    fn normalized(&self) -> Self;

    fn reverse(&mut self);

    fn reversed(&self) -> Self;

    fn rotate_by(&mut self, other: Self);

    fn rotated_by(&self, other: Self) -> Self;

    fn rotate_vec(&self, vec: &mut Self::Vector);

    fn into_matrix(&self) -> Self::Matrix;
}
