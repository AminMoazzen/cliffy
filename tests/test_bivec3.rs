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
