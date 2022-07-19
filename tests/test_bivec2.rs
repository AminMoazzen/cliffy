use cliffy::*;

#[test]
fn test_constructions() {
    assert_eq!(Bivec2::new(8.0), Bivec2 { xy: 8.0 });
    assert_eq!(Bivec2::one(), Bivec2 { xy: 1.0 });
    assert_eq!(Bivec2::zero(), Bivec2 { xy: 0.0 });
}

#[test]
fn test_add() {
    let bv1 = Bivec2::new(-1.0);
    let bv2 = Bivec2::new(1.0);

    assert_eq!(bv1 + bv2, Bivec2::new(0.0));

    let mut v3 = Bivec2::zero();
    v3 += bv2;
    assert_eq!(v3, bv2);
}

#[test]
fn test_sub() {
    let bv1 = Bivec2::new(-1.0);
    let bv2 = Bivec2::new(1.0);

    assert_eq!(bv1 - bv2, Bivec2::new(-2.0));

    let mut v3 = Bivec2::zero();
    v3 -= bv2;
    assert_eq!(v3, -bv2);
}

#[test]
fn test_mul() {
    let mut bv = Bivec2::new(-1.0);
    let f = 5.0;

    assert_eq!(bv * f, Bivec2::new(-5.0));
    assert_eq!(f * bv, Bivec2::new(-5.0));

    bv *= 1.0;
    assert_eq!(bv, bv);
}

#[test]
fn test_div() {
    let mut bv = Bivec2::new(-1.0);
    let f = 5.0;

    assert_eq!(bv / f, Bivec2::new(-0.2));
    assert_eq!(f / bv, Bivec2::new(-5.0));

    bv /= 1.0;
    assert_eq!(bv, bv);
}

#[test]
fn test_neg() {
    let bv = Bivec2::new(-1.0);

    assert_eq!(-bv, Bivec2::new(1.0));
}

#[test]
fn test_mag() {
    let bv = Bivec2::new(3.0);

    assert_eq!(bv.mag(), 3.0);
}

#[test]
fn test_mag_sq() {
    let bv = Bivec2::new(3.0);

    assert_eq!(bv.mag_sq(), 9.0);
}

#[test]
fn test_dot() {
    let bv1 = Bivec2::new(3.0);
    let bv2 = Bivec2::new(2.00);

    assert_eq!(bv1.dot(bv2), 6.0);
}

#[test]
fn test_normalize() {
    let mut bv = Bivec2::new(3.0);
    bv.normalize();

    assert_eq!(bv, Bivec2::new(1.0));
}

#[test]
fn test_normalized() {
    let bv = Bivec2::new(3.0);

    assert_eq!(bv.normalized(), Bivec2::new(1.0));
}
