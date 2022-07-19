use cliffy::*;

#[test]
fn test_constructions() {
    assert_eq!(
        Vec4::new(1.0, 3.0, 5.0, 7.0),
        Vec4 {
            x: 1.0,
            y: 3.0,
            z: 5.0,
            w: 7.0
        }
    );
    assert_eq!(
        Vec4::uni(8.0),
        Vec4 {
            x: 8.0,
            y: 8.0,
            z: 8.0,
            w: 8.0
        }
    );
    assert_eq!(
        Vec4::one(),
        Vec4 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0
        }
    );
    assert_eq!(
        Vec4::zero(),
        Vec4 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0
        }
    );
}

#[test]
fn test_add() {
    let v1 = Vec4::new(-1.0, 1.0, 5.0, 8.0);
    let v2 = Vec4::new(1.0, 2.0, -3.0, -4.0);

    assert_eq!(v1 + v2, Vec4::new(0.0, 3.0, 2.0, 4.0));

    let mut v3 = Vec4::zero();
    v3 += v2;
    assert_eq!(v3, v2);
}

#[test]
fn test_sub() {
    let v1 = Vec4::new(-1.0, 2.0, 5.0, 8.0);
    let v2 = Vec4::new(1.0, 1.0, 3.0, 4.0);

    assert_eq!(v1 - v2, Vec4::new(-2.0, 1.0, 2.0, 4.0));

    let mut v3 = Vec4::zero();
    v3 -= v2;
    assert_eq!(v3, -v2);
}

#[test]
fn test_mul() {
    let mut v = Vec4::new(-1.0, 1.0, 3.0, 2.0);
    let f = 5.0;

    assert_eq!(v * f, Vec4::new(-5.0, 5.0, 15.0, 10.0));
    assert_eq!(f * v, Vec4::new(-5.0, 5.0, 15.0, 10.0));

    v *= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_div() {
    let mut v = Vec4::new(-1.0, 1.0, 5.0, 10.0);
    let f = 5.0;

    assert_eq!(v / f, Vec4::new(-0.2, 0.2, 1.0, 2.0));
    assert_eq!(f / v, Vec4::new(-5.0, 5.0, 1.0, 0.5));

    v /= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_neg() {
    let v = Vec4::new(-1.0, 1.0, 5.0, -8.0);

    assert_eq!(-v, Vec4::new(1.0, -1.0, -5.0, 8.0));
}

#[test]
fn test_indexing() {
    let v = Vec4::new(-2.0, 3.0, 5.0, 8.0);
    assert_eq!(v[0], -2.0);
    assert_eq!(v[1], 3.0);
    assert_eq!(v[2], 5.0);
    assert_eq!(v[3], 8.0);

    let mut v = v;

    v[0] = 1.0;
    v[1] = -2.0;
    v[2] = 3.0;
    v[3] = 5.0;
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], -2.0);
    assert_eq!(v[2], 3.0);
    assert_eq!(v[3], 5.0);
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let v = Vec4::one();

    let _ = v[4];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut v = Vec4::one();

    v[4] = 5.0;
}

#[test]
fn test_conversions() {
    let v = Vec4::new(-2.0, 3.0, 5.0, 8.0);
    let a: [f32; 4] = v.into();
    assert_eq!(a, [-2.0, 3.0, 5.0, 8.0]);

    let mut a = [-2.0, 3.0, 5.0, 8.0];

    let v: Vec4 = a.into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let v: Vec4 = (&a).into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let v: Vec4 = (&mut a).into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let mut t = (-2.0, 3.0, 5.0, 8.0);

    let v: Vec4 = t.into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let v: Vec4 = (&t).into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let v: Vec4 = (&mut t).into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 5.0, 8.0));

    let v = Vec4::new(-2.0, 3.0, 5.0, 8.0);
    let t: (f32, f32) = v.into();
    assert_eq!(t, (-2.0, 3.0));

    let v2 = Vec2 { x: -2.0, y: 3.0 };
    let v: Vec4 = v2.into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 0.0, 0.0));

    let v3 = Vec3 {
        x: -2.0,
        y: 3.0,
        z: 8.0,
    };
    let v: Vec4 = v3.into();
    assert_eq!(v, Vec4::new(-2.0, 3.0, 8.0, 0.0));
}

#[test]
fn test_mag() {
    let v = Vec4::new(1.0, 2.0, 8.0, 10.0);

    assert_eq!(v.mag(), 13.0);
}

#[test]
fn test_mag_sq() {
    let v = Vec4::new(1.0, 2.0, 8.0, 10.0);

    assert_eq!(v.mag_sq(), 169.0);
}

#[test]
fn test_dot() {
    let v1 = Vec4::new(3.0, 4.0, 5.0, 7.0);
    let v2 = Vec4::new(2.0, 1.0, -3.0, 6.0);

    assert_eq!(v1.dot(v2), 37.0);
}

#[test]
fn test_normalize() {
    let mut v = Vec4::new(1.0, 2.0, 8.0, 10.0);
    let mag = 13.0;
    v.normalize();

    assert_eq!(v, Vec4::new(1.0 / mag, 2.0 / mag, 8.0 / mag, 10.0 / mag));
}

#[test]
fn test_normalized() {
    let mut v = Vec4::new(1.0, 2.0, 8.0, 10.0);
    let mag = 13.0;

    assert_eq!(
        v.normalized(),
        Vec4::new(1.0 / mag, 2.0 / mag, 8.0 / mag, 10.0 / mag)
    );
}

#[test]
fn test_project() {
    let v1 = Vec4::new(5.0, 0.0, 0.0, 0.0);
    let mut v2 = Vec4::new(2.0, 3.0, 8.0, -1.0);
    v2.project(v1);

    assert_eq!(v2, Vec4::new(2.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_projected() {
    let v1 = Vec4::new(5.0, 0.0, 0.0, 0.0);
    let mut v2 = Vec4::new(2.0, 3.0, 8.0, -1.0);

    assert_eq!(v2.projected(v1), Vec4::new(2.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_reject() {
    let v1 = Vec4::new(5.0, 0.0, 0.0, 0.0);
    let mut v2 = Vec4::new(2.0, 3.0, 8.0, -1.0);
    v2.reject(v1);

    assert_eq!(v2, Vec4::new(0.0, 3.0, 8.0, -1.0));
}

#[test]
fn test_rejected() {
    let v1 = Vec4::new(5.0, 0.0, 0.0, 0.0);
    let mut v2 = Vec4::new(2.0, 3.0, 8.0, -1.0);

    assert_eq!(v2.rejected(v1), Vec4::new(0.0, 3.0, 8.0, -1.0));
}

#[test]
fn test_reflect() {
    let v1 = Vec4::new(0.0, 10.0, 0.0, 0.0);
    let mut v2 = Vec4::new(1.0, -1.0, 1.0, 0.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec4::new(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn test_reflected() {
    let v1 = Vec4::new(0.0, 10.0, 0.0, 0.0);
    let mut v2 = Vec4::new(1.0, -1.0, 1.0, 0.0);

    assert_eq!(v2.reflected(v1), Vec4::new(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn test_reflect_normal() {
    let v1 = Vec4::new(0.0, 1.0, 0.0, 0.0);
    let mut v2 = Vec4::new(1.0, -1.0, 1.0, 0.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec4::new(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn test_reflected_normal() {
    let v1 = Vec4::new(0.0, 1.0, 0.0, 0.0);
    let v2 = Vec4::new(1.0, -1.0, 1.0, 0.0);

    assert_eq!(v2.reflected(v1), Vec4::new(1.0, 1.0, 1.0, 0.0));
}

#[test]
fn test_to() {
    let v1 = Vec4::new(5.0, 3.0, 1.0, 4.0);
    let v2 = Vec4::new(-1.0, 7.0, 8.0, 2.0);

    assert_eq!(v1.to(v2), Vec4::new(-6.0, 4.0, 7.0, -2.0));
}

#[test]
fn test_distance() {
    let v1 = Vec4::new(5.0, 3.0, 1.0, 4.0);
    let v2 = Vec4::new(-1.0, 7.0, 8.0, 2.0);

    assert_eq!(v1.distance(v2), Vec4::new(-6.0, 4.0, 7.0, -2.0).mag());
}

#[test]
fn test_lerp() {
    let v1 = Vec4::zero();
    let v2 = Vec4::one();

    assert_eq!(v1.lerp(v2, 0.0), v1);
    assert_eq!(v1.lerp(v2, 1.0), v2);
    assert_eq!(v1.lerp(v2, -0.5), Vec4::new(-0.5, -0.5, -0.5, -0.5));
    assert_eq!(v1.lerp(v2, 1.5), Vec4::new(1.5, 1.5, 1.5, 1.5));
}
