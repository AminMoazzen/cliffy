use cliffy::*;

#[test]
fn test_constructions() {
    assert_eq!(
        Bivec3::new(1.0, 3.0, 5.0),
        Bivec3 {
            xy: 1.0,
            xz: 3.0,
            yz: 5.0
        }
    );
    assert_eq!(
        Bivec3::one(),
        Bivec3 {
            xy: 1.0,
            xz: 1.0,
            yz: 1.0
        }
    );
    assert_eq!(
        Bivec3::zero(),
        Bivec3 {
            xy: 0.0,
            xz: 0.0,
            yz: 0.0
        }
    );
}

#[test]
fn test_add() {
    let bv1 = Bivec3::new(-1.0, 1.0, 5.0);
    let bv2 = Bivec3::new(1.0, 2.0, -3.0);

    assert_eq!(bv1 + bv2, Bivec3::new(0.0, 3.0, 2.0));

    let mut v3 = Bivec3::zero();
    v3 += bv2;
    assert_eq!(v3, bv2);
}

#[test]
fn test_sub() {
    let bv1 = Bivec3::new(-1.0, 2.0, 5.0);
    let bv2 = Bivec3::new(1.0, 1.0, 3.0);

    assert_eq!(bv1 - bv2, Bivec3::new(-2.0, 1.0, 2.0));

    let mut v3 = Bivec3::zero();
    v3 -= bv2;
    assert_eq!(v3, -bv2);
}

#[test]
fn test_mul() {
    let mut bv = Bivec3::new(-1.0, 1.0, 3.0);
    let f = 5.0;

    assert_eq!(bv * f, Bivec3::new(-5.0, 5.0, 15.0));
    assert_eq!(f * bv, Bivec3::new(-5.0, 5.0, 15.0));

    bv *= 1.0;
    assert_eq!(bv, bv);
}

#[test]
fn test_div() {
    let mut bv = Bivec3::new(-1.0, 1.0, 5.0);
    let f = 5.0;

    assert_eq!(bv / f, Bivec3::new(-0.2, 0.2, 1.0));
    assert_eq!(f / bv, Bivec3::new(-5.0, 5.0, 1.0));

    bv /= 1.0;
    assert_eq!(bv, bv);
}

#[test]
fn test_neg() {
    let bv = Bivec3::new(-1.0, 1.0, 5.0);

    assert_eq!(-bv, Bivec3::new(1.0, -1.0, -5.0));
}

#[test]
fn test_mag() {
    let bv = Bivec3::new(3.0, 4.0, 12.0);

    assert_eq!(bv.mag(), 13.0);
}

#[test]
fn test_mag_sq() {
    let bv = Bivec3::new(3.0, 4.0, 12.0);

    assert_eq!(bv.mag_sq(), 169.0);
}

#[test]
fn test_dot() {
    let bv1 = Bivec3::new(3.0, 4.0, 5.0);
    let bv2 = Bivec3::new(2.0, 1.0, -3.0);

    assert_eq!(bv1.dot(bv2), -5.0);
}

#[test]
fn test_normalize() {
    let mut bv = Bivec3::new(3.0, 4.0, 12.0);
    let mag = 13.0;
    bv.normalize();

    assert_eq!(bv, Bivec3::new(3.0 / mag, 4.0 / mag, 12.0 / mag));
}

#[test]
fn test_normalized() {
    let bv = Bivec3::new(3.0, 4.0, 12.0);
    let mag = 13.0;

    assert_eq!(
        bv.normalized(),
        Bivec3::new(3.0 / mag, 4.0 / mag, 12.0 / mag)
    );
}

#[test]
fn test_bivec_mul() {
    let bv1 = Bivec3::new(2.0, 3.0, 4.0);
    let bv2 = Bivec3::new(3.0, 2.0, 5.0);

    assert_eq!(bv1 * bv2, Bivec3::new(6.0, 6.0, 20.0));
}

#[test]
fn test_bivec_div() {
    let bv1 = Bivec3::new(6.0, 8.0, 12.0);
    let bv2 = Bivec3::new(2.0, 2.0, 3.0);

    assert_eq!(bv1 / bv2, Bivec3::new(3.0, 4.0, 4.0));
}

#[test]
fn test_scalar_mul_bivec() {
    let bv = Bivec3::new(2.0, 3.0, 4.0);
    let f = 3.0;

    assert_eq!(f * bv, Bivec3::new(6.0, 9.0, 12.0));
    assert_eq!(bv * f, Bivec3::new(6.0, 9.0, 12.0));
}

#[test]
fn test_scalar_div_bivec() {
    let bv = Bivec3::new(2.0, 4.0, 8.0);
    let f = 2.0;

    assert_eq!(f / bv, Bivec3::new(1.0, 0.5, 0.25));
    assert_eq!(bv / f, Bivec3::new(1.0, 2.0, 4.0));
}

#[test]
fn test_mul_assign_bivec() {
    let mut bv1 = Bivec3::new(2.0, 3.0, 4.0);
    let bv2 = Bivec3::new(3.0, 2.0, 5.0);

    bv1 *= bv2;
    assert_eq!(bv1, Bivec3::new(6.0, 6.0, 20.0));
}

#[test]
fn test_mul_assign_scalar() {
    let mut bv = Bivec3::new(2.0, 3.0, 4.0);

    bv *= 3.0;
    assert_eq!(bv, Bivec3::new(6.0, 9.0, 12.0));
}

#[test]
fn test_div_assign_bivec() {
    let mut bv1 = Bivec3::new(6.0, 8.0, 12.0);
    let bv2 = Bivec3::new(2.0, 2.0, 3.0);

    bv1 /= bv2;
    assert_eq!(bv1, Bivec3::new(3.0, 4.0, 4.0));
}

#[test]
fn test_div_assign_scalar() {
    let mut bv = Bivec3::new(6.0, 8.0, 12.0);

    bv /= 2.0;
    assert_eq!(bv, Bivec3::new(3.0, 4.0, 6.0));
}

#[test]
fn test_zero_magnitude() {
    let bv = Bivec3::zero();

    assert_eq!(bv.mag(), 0.0);
    assert_eq!(bv.mag_sq(), 0.0);
}

#[test]
fn test_orthogonal_bivectors() {
    let bv1 = Bivec3::new(1.0, 0.0, 0.0);
    let bv2 = Bivec3::new(0.0, 1.0, 0.0);

    assert_eq!(bv1.dot(bv2), 0.0);
}

#[test]
fn test_parallel_bivectors() {
    let bv1 = Bivec3::new(2.0, 3.0, 4.0);
    let bv2 = Bivec3::new(4.0, 6.0, 8.0);

    assert_eq!(bv1.dot(bv2), 8.0 + 18.0 + 32.0);
}
