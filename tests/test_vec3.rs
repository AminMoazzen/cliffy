use cliffy::*;

#[test]
fn test_constructions() {
    assert_eq!(
        Vec3::new(1.0, 3.0, 5.0),
        Vec3 {
            x: 1.0,
            y: 3.0,
            z: 5.0
        }
    );
    assert_eq!(
        Vec3::uni(8.0),
        Vec3 {
            x: 8.0,
            y: 8.0,
            z: 8.0
        }
    );
    assert_eq!(
        Vec3::one(),
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    );
    assert_eq!(
        Vec3::zero(),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    );
    assert_eq!(
        Vec3::up(),
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    );
    assert_eq!(
        Vec3::down(),
        Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0
        }
    );
    assert_eq!(
        Vec3::right(),
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    );
    assert_eq!(
        Vec3::left(),
        Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0
        }
    );
    assert_eq!(
        Vec3::forward(),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    );
    assert_eq!(
        Vec3::back(),
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0
        }
    );
}

#[test]
fn test_add() {
    let v1 = Vec3::new(-1.0, 1.0, 5.0);
    let v2 = Vec3::new(1.0, 2.0, -3.0);

    assert_eq!(v1 + v2, Vec3::new(0.0, 3.0, 2.0));

    let mut v3 = Vec3::zero();
    v3 += v2;
    assert_eq!(v3, v2);
}

#[test]
fn test_sub() {
    let v1 = Vec3::new(-1.0, 2.0, 5.0);
    let v2 = Vec3::new(1.0, 1.0, 3.0);

    assert_eq!(v1 - v2, Vec3::new(-2.0, 1.0, 2.0));

    let mut v3 = Vec3::zero();
    v3 -= v2;
    assert_eq!(v3, -v2);
}

#[test]
fn test_mul() {
    let mut v = Vec3::new(-1.0, 1.0, 3.0);
    let f = 5.0;

    assert_eq!(v * f, Vec3::new(-5.0, 5.0, 15.0));
    assert_eq!(f * v, Vec3::new(-5.0, 5.0, 15.0));

    v *= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_div() {
    let mut v = Vec3::new(-1.0, 1.0, 5.0);
    let f = 5.0;

    assert_eq!(v / f, Vec3::new(-0.2, 0.2, 1.0));
    assert_eq!(f / v, Vec3::new(-5.0, 5.0, 1.0));

    v /= 1.0;
    assert_eq!(v, v);
}

#[test]
fn test_neg() {
    let v = Vec3::new(-1.0, 1.0, 5.0);

    assert_eq!(-v, Vec3::new(1.0, -1.0, -5.0));
}

#[test]
fn test_indexing() {
    let v = Vec3::new(-2.0, 3.0, 5.0);
    assert_eq!(v[0], -2.0);
    assert_eq!(v[1], 3.0);
    assert_eq!(v[2], 5.0);

    let mut v = v;

    v[0] = 1.0;
    v[1] = -2.0;
    v[2] = 3.0;
    assert_eq!(v[0], 1.0);
    assert_eq!(v[1], -2.0);
    assert_eq!(v[2], 3.0);
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let v = Vec3::one();

    let _ = v[3];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut v = Vec3::one();

    v[3] = 5.0;
}

#[test]
fn test_conversions() {
    let v = Vec3::new(-2.0, 3.0, 5.0);
    let a: [f32; 3] = v.into();
    assert_eq!(a, [-2.0, 3.0, 5.0]);

    let mut a = [-2.0, 3.0, 5.0];

    let v: Vec3 = a.into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let v: Vec3 = (&a).into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let v: Vec3 = (&mut a).into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let mut t = (-2.0, 3.0, 5.0);

    let v: Vec3 = t.into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let v: Vec3 = (&t).into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let v: Vec3 = (&mut t).into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 5.0));

    let v = Vec3::new(-2.0, 3.0, 5.0);
    let t: (f32, f32) = v.into();
    assert_eq!(t, (-2.0, 3.0));

    let v2 = Vec2 { x: -2.0, y: 3.0 };
    let v: Vec3 = v2.into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 0.0));

    let v4 = Vec4 {
        x: -2.0,
        y: 3.0,
        z: 8.0,
        w: 12.0,
    };
    let v: Vec3 = v4.into();
    assert_eq!(v, Vec3::new(-2.0, 3.0, 8.0));
}

#[test]
fn test_mag() {
    let v = Vec3::new(3.0, 4.0, 12.0);

    assert_eq!(v.mag(), 13.0);
}

#[test]
fn test_mag_sq() {
    let v = Vec3::new(3.0, 4.0, 12.0);

    assert_eq!(v.mag_sq(), 169.0);
}

#[test]
fn test_dot() {
    let v1 = Vec3::new(3.0, 4.0, 5.0);
    let v2 = Vec3::new(2.0, 1.0, -3.0);

    assert_eq!(v1.dot(v2), -5.0);

    assert_eq!(Vec3::up().dot(Vec3::right()), 0.0);
    assert_eq!(Vec3::up().dot(Vec3::up()), 1.0);
    assert_eq!(Vec3::up().dot(Vec3::down()), -1.0);
}

#[test]
fn test_wedge() {
    let v1 = Vec3::new(3.0, 4.0, 5.0);
    let v2 = Vec3::new(2.0, 1.0, 6.0);
    let w = Bivec3 {
        xy: -5.0,
        xz: 8.0,
        yz: 19.0,
    };

    assert_eq!(v1.wedge(v2), w);
    assert_eq!(v2.wedge(v1), -w);

    assert_eq!(
        Vec3::up().wedge(Vec3::up()),
        Bivec3 {
            xy: 0.0,
            xz: 0.0,
            yz: 0.0
        }
    );
    assert_eq!(
        Vec3::up().wedge(Vec3::right()),
        Bivec3 {
            xy: -1.0,
            xz: 0.0,
            yz: 0.0
        }
    );
    assert_eq!(
        Vec3::up().wedge(Vec3::left()),
        Bivec3 {
            xy: 1.0,
            xz: 0.0,
            yz: 0.0
        }
    );
    assert_eq!(
        Vec3::up().wedge(Vec3::forward()),
        Bivec3 {
            xy: 0.0,
            xz: 0.0,
            yz: 1.0
        }
    );
    assert_eq!(
        Vec3::up().wedge(Vec3::back()),
        Bivec3 {
            xy: 0.0,
            xz: 0.0,
            yz: -1.0
        }
    );
    assert_eq!(
        Vec3::right().wedge(Vec3::forward()),
        Bivec3 {
            xy: 0.0,
            xz: 1.0,
            yz: 0.0
        }
    );
    assert_eq!(
        Vec3::right().wedge(Vec3::back()),
        Bivec3 {
            xy: 0.0,
            xz: -1.0,
            yz: 0.0
        }
    );
}

#[test]
fn test_geom() {
    let v1 = Vec3::new(3.0, 4.0, 5.0);
    let v2 = Vec3::new(2.0, 1.0, 6.0);
    let r = Rot3 {
        s: 40.0,
        bv: Bivec3 {
            xy: -5.0,
            xz: 8.0,
            yz: 19.0,
        },
    };

    assert_eq!(v1.geom(v2), r);

    let r = Rot3 {
        s: v1.mag_sq(),
        bv: Bivec3 {
            xy: 0.0,
            xz: 0.0,
            yz: 0.0,
        },
    };
    assert_eq!(v1.geom(v1), r);

    let r = Rot3 {
        s: 0.0,
        bv: Vec3::up().wedge(Vec3::right()),
    };
    assert_eq!(Vec3::up().geom(Vec3::right()), r);
}

#[test]
fn test_cross() {
    let v1 = Vec3::new(3.0, 4.0, 5.0);
    let v2 = Vec3::new(2.0, 1.0, 6.0);
    let c = Vec3::new(19.0, -8.0, -5.0);

    assert_eq!(v1.cross(v2), c);
    assert_eq!(v2.cross(v1), -c);

    assert_eq!(Vec3::up().cross(Vec3::up()), Vec3::zero());
    assert_eq!(Vec3::up().cross(Vec3::right()), Vec3::back());
    assert_eq!(Vec3::up().cross(Vec3::left()), Vec3::forward());
    assert_eq!(Vec3::up().cross(Vec3::forward()), Vec3::right());
    assert_eq!(Vec3::up().cross(Vec3::back()), Vec3::left());
    assert_eq!(Vec3::right().cross(Vec3::forward()), Vec3::down());
    assert_eq!(Vec3::right().cross(Vec3::back()), Vec3::up());
}

#[test]
fn test_normalize() {
    let mut v = Vec3::new(3.0, 4.0, 12.0);
    let mag = 13.0;
    v.normalize();

    assert_eq!(v, Vec3::new(3.0 / mag, 4.0 / mag, 12.0 / mag));
}

#[test]
fn test_normalized() {
    let v = Vec3::new(3.0, 4.0, 12.0);
    let mag = 13.0;

    assert_eq!(v.normalized(), Vec3::new(3.0 / mag, 4.0 / mag, 12.0 / mag));
}

#[test]
fn test_project() {
    let v1 = Vec3::new(5.0, 0.0, 0.0);
    let mut v2 = Vec3::new(2.0, 3.0, 8.0);
    v2.project(v1);

    assert_eq!(v2, Vec3::new(2.0, 0.0, 0.0));
}

#[test]
fn test_projected() {
    let v1 = Vec3::new(5.0, 0.0, 0.0);
    let v2 = Vec3::new(2.0, 3.0, 8.0);

    assert_eq!(v2.projected(v1), Vec3::new(2.0, 0.0, 0.0));
}

#[test]
fn test_reject() {
    let v1 = Vec3::new(5.0, 0.0, 0.0);
    let mut v2 = Vec3::new(2.0, 3.0, 8.0);
    v2.reject(v1);

    assert_eq!(v2, Vec3::new(0.0, 3.0, 8.0));
}

#[test]
fn test_rejected() {
    let v1 = Vec3::new(5.0, 0.0, 0.0);
    let v2 = Vec3::new(2.0, 3.0, 8.0);

    assert_eq!(v2.rejected(v1), Vec3::new(0.0, 3.0, 8.0));
}

#[test]
fn test_reflect() {
    let v1 = Vec3::new(0.0, 10.0, 0.0);
    let mut v2 = Vec3::new(1.0, -1.0, 1.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_reflected() {
    let v1 = Vec3::new(0.0, 10.0, 0.0);
    let v2 = Vec3::new(1.0, -1.0, 1.0);

    assert_eq!(v2.reflected(v1), Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_reflect_normal() {
    let v1 = Vec3::up();
    let mut v2 = Vec3::new(1.0, -1.0, 1.0);
    v2.reflect(v1);

    assert_eq!(v2, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_reflected_normal() {
    let v1 = Vec3::up();
    let v2 = Vec3::new(1.0, -1.0, 1.0);

    assert_eq!(v2.reflected(v1), Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_to() {
    let v1 = Vec3::new(5.0, 3.0, 1.0);
    let v2 = Vec3::new(-1.0, 7.0, 8.0);

    assert_eq!(v1.to(v2), Vec3::new(-6.0, 4.0, 7.0));
}

#[test]
fn test_distance() {
    let v1 = Vec3::new(5.0, 3.0, 1.0);
    let v2 = Vec3::new(-1.0, 7.0, 8.0);

    assert_eq!(v1.distance(v2), Vec3::new(-6.0, 4.0, 7.0).mag());
}

#[test]
fn test_angle_between() {
    let mut v1 = Vec3::new(10.0, 10.0, 0.0);
    let mut v2 = Vec3::new(-5.0, 5.0, 0.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec3::new(-10.0, 0.0, 10.0);
    v2 = Vec3::new(-5.0, 0.0, -5.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec3::new(0.0, -10.0, 10.0);
    v2 = Vec3::new(0.0, -5.0, -5.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);
}

#[test]
fn test_angle_between_normal() {
    let mut v1 = Vec3::new(1.0, 0.0, 0.0);
    let mut v2 = Vec3::new(0.0, 1.0, 0.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec3::new(-1.0, 0.0, 0.0);
    v2 = Vec3::new(0.0, 0.0, -1.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);

    v1 = Vec3::new(0.0, 1.0, 0.0);
    v2 = Vec3::new(0.0, 0.0, -1.0);
    assert_eq!(v1.angle_between(v2), std::f32::consts::PI / 2.0);
}

#[test]
fn test_lerp() {
    let v1 = Vec3::zero();
    let v2 = Vec3::one();

    assert_eq!(v1.lerp(v2, 0.0), v1);
    assert_eq!(v1.lerp(v2, 1.0), v2);
    assert_eq!(v1.lerp(v2, -0.5), Vec3::new(-0.5, -0.5, -0.5));
    assert_eq!(v1.lerp(v2, 1.5), Vec3::new(1.5, 1.5, 1.5));
}

#[test]
fn test_slerp() {
    let v1 = Vec3::new(-1.0, 1.0, 1.0);
    let v2 = Vec3::new(1.0, 1.0, -1.0);

    assert_eq!(v1.slerp(v2, 0.0), v1);
    assert_eq!(v1.slerp(v2, 0.5), Vec3::new(0.0, 3f32.sqrt(), 0.0));
    assert_eq!(v1.slerp(v2, 1.0), v2);
}

#[test]
fn test_nlerp() {
    let v1 = Vec3::new(-1.0, 1.0, 1.0);
    let v2 = Vec3::new(1.0, 1.0, -1.0);

    assert_eq!(v1.nlerp(v2, 0.0), v1.normalized());
    assert_eq!(v1.nlerp(v2, 0.5), Vec3::up());
    assert_eq!(v1.nlerp(v2, 1.0), v2.normalized());
}
