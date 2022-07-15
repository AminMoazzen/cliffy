use cliffy::*;

#[test]
fn test_constructions() {
    assert_eq!(Vec2::new(1.0, 3.0), Vec2 { x: 1.0, y: 3.0 });
    assert_eq!(Vec2::uni(8.0), Vec2 { x: 8.0, y: 8.0 });
    assert_eq!(Vec2::one(), Vec2 { x: 1.0, y: 1.0 });
    assert_eq!(Vec2::zero(), Vec2 { x: 0.0, y: 0.0 });
    assert_eq!(Vec2::up(), Vec2 { x: 0.0, y: 1.0 });
    assert_eq!(Vec2::down(), Vec2 { x: 0.0, y: -1.0 });
    assert_eq!(Vec2::right(), Vec2 { x: 1.0, y: 0.0 });
    assert_eq!(Vec2::left(), Vec2 { x: -1.0, y: 0.0 });
}

#[test]
fn test_add() {
    let v1 = Vec2::new(-1.0, 1.0);
    let v2 = Vec2::new(1.0, 2.0);

    assert_eq!(v1 + v2, Vec2::new(0.0, 3.0));

    let mut v3 = Vec2::zero();
    v3 += v2;
    assert_eq!(v3, v2);
}

#[test]
fn test_sub() {
    let v1 = Vec2::new(-1.0, 2.0);
    let v2 = Vec2::new(1.0, 1.0);

    assert_eq!(v1 - v2, Vec2::new(-2.0, 1.0));

    let mut v3 = Vec2::zero();
    v3 -= v2;
    assert_eq!(v3, -v2);
}

#[test]
fn test_mul() {
    let mut v = Vec2::new(-1.0, 1.0);
    let f = 5.0;

    assert_eq!(v * f, Vec2::new(-5.0, 5.0));
    assert_eq!(f * v, Vec2::new(-5.0, 5.0));

    v *= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_div() {
    let mut v = Vec2::new(-1.0, 1.0);
    let f = 5.0;

    assert_eq!(v / f, Vec2::new(-0.2, 0.2));
    assert_eq!(f / v, Vec2::new(-5.0, 5.0));

    v /= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_neg() {
    let v = Vec2::new(-1.0, 1.0);

    assert_eq!(-v, Vec2::new(1.0, -1.0));
}

#[test]
fn test_indexing() {
    let v = Vec2::new(-2.0, 3.0);
    assert_eq!(v[0], -2.0);
    assert_eq!(v[1], 3.0);

    let mut v = v;

    v[0] = 1.0;
    v[1] = -2.0;
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], -2.0);
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let v = Vec2::one();

    let _ = v[3];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut v = Vec2::one();

    v[3] = 5.0;
}

#[test]
fn test_conversions() {
    let v = Vec2::new(-2.0, 3.0);
    let a: [f32; 2] = v.into();
    assert_eq!(a, [-2.0, 3.0]);

    let mut a = [-2.0, 3.0];

    let v: Vec2 = a.into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v: Vec2 = (&a).into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v: Vec2 = (&mut a).into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let mut t = (-2.0, 3.0);

    let v: Vec2 = t.into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v: Vec2 = (&t).into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v: Vec2 = (&mut t).into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v = Vec2::new(-2.0, 3.0);
    let t: (f32, f32) = v.into();
    assert_eq!(t, (-2.0, 3.0));

    let v3 = Vec3 {
        x: -2.0,
        y: 3.0,
        z: 8.0,
    };
    let v: Vec2 = v3.into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));

    let v4 = Vec4 {
        x: -2.0,
        y: 3.0,
        z: 8.0,
        w: 12.0,
    };
    let v: Vec2 = v4.into();
    assert_eq!(v, Vec2::new(-2.0, 3.0));
}

#[test]
fn test_mag() {
    let v = Vec2::new(3.0, 4.0);

    assert_eq!(v.mag(), 5.0);
}

#[test]
fn test_mag_sq() {
    let v = Vec2::new(3.0, 4.0);

    assert_eq!(v.mag_sq(), 25.0);
}

#[test]
fn test_dot() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(2.0, 1.0);

    assert_eq!(v1.dot(v2), 10.0);

    assert_eq!(Vec2::up().dot(Vec2::right()), 0.0);
    assert_eq!(Vec2::up().dot(Vec2::up()), 1.0);
    assert_eq!(Vec2::up().dot(Vec2::down()), -1.0);
}

#[test]
fn test_wedge() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(2.0, 1.0);
    let w = Bivec2 { xy: -5.0 };

    assert_eq!(v1.wedge(v2), w);
    assert_eq!(v2.wedge(v1), -w);

    assert_eq!(Vec2::up().wedge(Vec2::right()), Bivec2 { xy: -1.0 });
    assert_eq!(Vec2::up().wedge(Vec2::up()), Bivec2 { xy: 0.0 });
    assert_eq!(Vec2::up().wedge(Vec2::left()), Bivec2 { xy: 1.0 });
}

#[test]
fn test_geom() {
    let v1 = Vec2::new(3.0, 4.0);
    let v2 = Vec2::new(2.0, 1.0);
    let r = Rot2 {
        s: 10.0,
        bv: Bivec2 { xy: -5.0 },
    };

    assert_eq!(v1.geom(v2), r);

    let r = Rot2 {
        s: v1.mag_sq(),
        bv: Bivec2 { xy: 0.0 },
    };
    assert_eq!(v1.geom(v1), r);

    let r = Rot2 {
        s: 0.0,
        bv: Vec2::up().wedge(Vec2::right()),
    };
    assert_eq!(Vec2::up().geom(Vec2::right()), r);
}

#[test]
fn test_normalize() {
    let mut v = Vec2::new(3.0, 4.0);
    let mag = 5.0;
    v.normalize();

    assert_eq!(v, Vec2::new(3.0 / mag, 4.0 / mag));
}

#[test]
fn test_normalized() {
    let v = Vec2::new(3.0, 4.0);
    let mag = 5.0;

    assert_eq!(v.normalized(), Vec2::new(3.0 / mag, 4.0 / mag));
}

#[test]
fn test_project() {
    let v1 = Vec2::new(5.0, 0.0);
    let mut v2 = Vec2::new(2.0, 3.0);
    v2.project(v1);

    assert_eq!(v2, Vec2::new(2.0, 0.0));
}

#[test]
fn test_projected() {
    let v1 = Vec2::new(5.0, 0.0);
    let v2 = Vec2::new(2.0, 3.0);

    assert_eq!(v2.projected(v1), Vec2::new(2.0, 0.0));
}

#[test]
fn test_reject() {
    let v1 = Vec2::new(5.0, 0.0);
    let mut v2 = Vec2::new(2.0, 3.0);
    v2.reject(v1);

    assert_eq!(v2, Vec2::new(0.0, 3.0));
}

#[test]
fn test_rejected() {
    let v1 = Vec2::new(5.0, 0.0);
    let v2 = Vec2::new(2.0, 3.0);

    assert_eq!(v2.rejected(v1), Vec2::new(0.0, 3.0));
}

#[test]
fn test_reflect() {
    let v1 = Vec2::new(0.0, 10.0);
    let mut v2 = Vec2::new(1.0, -1.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec2::new(1.0, 1.0));
}

#[test]
fn test_reflected() {
    let v1 = Vec2::new(0.0, 10.0);
    let mut v2 = Vec2::new(1.0, -1.0);

    assert_eq!(v2.reflected(v1), Vec2::new(1.0, 1.0));
}

#[test]
fn test_reflect_normal() {
    let v1 = Vec2::up();
    let mut v2 = Vec2::new(1.0, -1.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec2::new(1.0, 1.0));
}

#[test]
fn test_reflected_normal() {
    let v1 = Vec2::up();
    let v2 = Vec2::new(1.0, -1.0);

    assert_eq!(v2.reflected(v1), Vec2::new(1.0, 1.0));
}

#[test]
fn test_to() {
    let v1 = Vec2::new(5.0, 3.0);
    let v2 = Vec2::new(-1.0, 7.0);

    assert_eq!(v1.to(v2), Vec2::new(-6.0, 4.0));
}

#[test]
fn test_distance() {
    let v1 = Vec2::new(5.0, 3.0);
    let v2 = Vec2::new(-1.0, 7.0);

    assert_eq!(v1.distance(v2), Vec2::new(-6.0, 4.0).mag());
}

#[test]
fn test_angle_between() {
    let mut v1 = Vec2::new(10.0, 10.0);
    let mut v2 = Vec2::new(-5.0, 5.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec2::new(-10.0, 10.0);
    v2 = Vec2::new(-5.0, -5.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec2::new(10.0, -10.0);
    v2 = Vec2::new(-5.0, -5.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);
}

#[test]
fn test_angle_between_normal() {
    let mut v1 = Vec2::new(1.0, 0.0);
    let mut v2 = Vec2::new(0.0, 1.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec2::new(-1.0, 0.0);
    v2 = Vec2::new(0.0, -1.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec2::new(1.0, 0.0);
    v2 = Vec2::new(0.0, -1.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);
}

#[test]
fn test_lerp() {
    let v1 = Vec2::zero();
    let v2 = Vec2::one();

    assert_eq!(v1.lerp(v2, 0.0), v1);
    assert_eq!(v1.lerp(v2, 1.0), v2);
    assert_eq!(v1.lerp(v2, -0.5), Vec2::new(-0.5, -0.5));
    assert_eq!(v1.lerp(v2, 1.5), Vec2::new(1.5, 1.5));
}

#[test]
fn test_slerp() {
    let v1 = Vec2::new(-1.0, 1.0);
    let v2 = Vec2::new(1.0, 1.0);
    assert_eq!(v1.slerp(v2, 0.0), v1);
    assert_eq!(v1.slerp(v2, 0.5), Vec2::new(0.0, 2f32.sqrt()));
    assert_eq!(v1.slerp(v2, 1.0), v2);
}

#[test]
fn test_nlerp() {
    let v1 = Vec2::new(-1.0, 1.0);
    let v2 = Vec2::new(1.0, 1.0);

    assert_eq!(v1.nlerp(v2, 0.0), v1.normalized());
    assert_eq!(v1.nlerp(v2, 0.5), Vec2::up());
    assert_eq!(v1.nlerp(v2, 1.0), v2.normalized());
}
