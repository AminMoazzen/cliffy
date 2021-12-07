pub trait Rotor {
    type Decimal;
    type Bivec;
    type Vector;

    fn rotate_vec(&self, vec: &mut Self::Vector);
}
