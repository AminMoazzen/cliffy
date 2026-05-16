use cliffy::*;

const EPS: f32 = 1e-5;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
    approx_eq(a.x, b.x) && approx_eq(a.y, b.y) && approx_eq(a.z, b.z)
}

#[test]
fn test_constructions() {
    let r = Rot3::new(0.5, Bivec3::new(0.1, 0.2, 0.3));
    assert_eq!(r.s, 0.5);
    assert_eq!(r.bv, Bivec3::new(0.1, 0.2, 0.3));

    let id = Rot3::identity();
    assert_eq!(id.s, 1.0);
    assert_eq!(id.bv, Bivec3::zero());
}

#[test]
fn test_from_angle_plane_zero() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(0.0, plane);
    assert!(approx_eq(r.s, 1.0));
    assert!(approx_eq(r.bv.xy, 0.0));
    assert!(approx_eq(r.bv.xz, 0.0));
    assert!(approx_eq(r.bv.yz, 0.0));
}

#[test]
fn test_from_angle_plane_xy() {
    let angle = std::f32::consts::PI / 2.0;
    let plane = Bivec3::new(1.0, 0.0, 0.0); // XY plane (normalized)
    let half = angle / 2.0;
    let (sin, cos) = half.sin_cos();
    let r = Rot3::from_angle_plane(angle, plane);
    assert!(approx_eq(r.s, cos));
    assert!(approx_eq(r.bv.xy, -sin));
    assert!(approx_eq(r.bv.xz, 0.0));
    assert!(approx_eq(r.bv.yz, 0.0));
}

#[test]
fn test_identity_rotate_vec() {
    let r = Rot3::identity();
    let v = Vec3::new(1.0, 2.0, 3.0);
    let rotated = r * v;
    assert!(vec3_approx_eq(rotated, v));
}

#[test]
fn test_rotate_90_in_xy_plane() {
    let plane = Bivec3::new(1.0, 0.0, 0.0); // XY plane
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    // (1, 0, 0) rotated 90° in XY plane → (0, 1, 0)
    let rotated = r * Vec3::right();
    assert!(vec3_approx_eq(rotated, Vec3::up()));
}

#[test]
fn test_rotate_90_in_xz_plane() {
    let plane = Bivec3::new(0.0, 1.0, 0.0); // XZ plane
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    // (1, 0, 0) rotated 90° in XZ plane → (0, 0, 1)
    let rotated = r * Vec3::right();
    assert!(vec3_approx_eq(rotated, Vec3::forward()));
}

#[test]
fn test_rotate_90_in_yz_plane() {
    let plane = Bivec3::new(0.0, 0.0, 1.0); // YZ plane
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    // (0, 1, 0) rotated 90° in YZ plane → (0, 0, 1)
    let rotated = r * Vec3::up();
    assert!(vec3_approx_eq(rotated, Vec3::forward()));
}

#[test]
fn test_rotate_preserves_magnitude() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 3.0, plane).normalized();
    let v = Vec3::new(3.0, 4.0, 5.0);
    let rotated = r * v;
    assert!(approx_eq(rotated.mag(), v.mag()));
}

#[test]
fn test_rotate_vec_direct() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    let mut v = Vec3::right();
    r.rotate_vec(&mut v);
    assert!(vec3_approx_eq(v, Vec3::up()));
}

#[test]
fn test_mag_identity() {
    let r = Rot3::identity();
    assert!(approx_eq(r.mag(), 1.0));
    assert!(approx_eq(r.mag_sq(), 1.0));
}

#[test]
fn test_mag() {
    let r = Rot3::new(2.0, Bivec3::new(1.0, 2.0, 2.0));
    // mag_sq = 4 + 1 + 4 + 4 = 13
    assert!(approx_eq(r.mag_sq(), 13.0));
    assert!(approx_eq(r.mag(), 13f32.sqrt()));
}

#[test]
fn test_normalize() {
    let mut r = Rot3::new(2.0, Bivec3::new(1.0, 2.0, 2.0));
    r.normalize();
    assert!(approx_eq(r.mag(), 1.0));
}

#[test]
fn test_normalized() {
    let r = Rot3::new(2.0, Bivec3::new(1.0, 2.0, 2.0));
    let rn = r.normalized();
    assert!(approx_eq(rn.mag(), 1.0));
}

#[test]
fn test_reverse() {
    let r = Rot3::new(1.0, Bivec3::new(2.0, 3.0, 4.0));
    let rev = r.reversed();
    assert_eq!(rev.s, r.s);
    assert_eq!(rev.bv.xy, -r.bv.xy);
    assert_eq!(rev.bv.xz, -r.bv.xz);
    assert_eq!(rev.bv.yz, -r.bv.yz);
}

#[test]
fn test_reverse_in_place() {
    let mut r = Rot3::new(1.0, Bivec3::new(2.0, 3.0, 4.0));
    r.reverse();
    assert_eq!(r.s, 1.0);
    assert_eq!(r.bv.xy, -2.0);
    assert_eq!(r.bv.xz, -3.0);
    assert_eq!(r.bv.yz, -4.0);
}

#[test]
fn test_reverse_undoes_rotation() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 4.0, plane).normalized();
    let v = Vec3::new(1.0, 2.0, 3.0);
    let forward = r * v;
    let back = r.reversed() * forward;
    assert!(vec3_approx_eq(back, v));
}

#[test]
fn test_mul_rotor_by_reverse_is_identity() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 4.0, plane).normalized();
    let product = r * r.reversed();
    assert!(approx_eq(product.s, 1.0));
    assert!(approx_eq(product.bv.xy, 0.0));
    assert!(approx_eq(product.bv.xz, 0.0));
    assert!(approx_eq(product.bv.yz, 0.0));
}

#[test]
fn test_mul_two_rotations_in_xy_plane() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    let combined = r * r;
    // Two 90° rotations in XY plane = 180°: (1,0,0) → (-1,0,0)
    let rotated = combined * Vec3::right();
    assert!(vec3_approx_eq(rotated, -Vec3::right()));
}

#[test]
fn test_add() {
    let r1 = Rot3::new(1.0, Bivec3::new(1.0, 2.0, 3.0));
    let r2 = Rot3::new(2.0, Bivec3::new(-1.0, 0.0, 1.0));
    let sum = r1 + r2;
    assert_eq!(sum.s, 3.0);
    assert_eq!(sum.bv.xy, 0.0);
    assert_eq!(sum.bv.xz, 2.0);
    assert_eq!(sum.bv.yz, 4.0);
}

#[test]
fn test_add_assign() {
    let mut r = Rot3::new(1.0, Bivec3::new(1.0, 2.0, 3.0));
    r += Rot3::new(2.0, Bivec3::new(-1.0, 0.0, 1.0));
    assert_eq!(r.s, 3.0);
    assert_eq!(r.bv.xy, 0.0);
    assert_eq!(r.bv.xz, 2.0);
    assert_eq!(r.bv.yz, 4.0);
}

#[test]
fn test_sub() {
    let r1 = Rot3::new(5.0, Bivec3::new(3.0, 2.0, 1.0));
    let r2 = Rot3::new(2.0, Bivec3::new(1.0, 1.0, 1.0));
    let diff = r1 - r2;
    assert_eq!(diff.s, 3.0);
    assert_eq!(diff.bv.xy, 2.0);
    assert_eq!(diff.bv.xz, 1.0);
    assert_eq!(diff.bv.yz, 0.0);
}

#[test]
fn test_sub_assign() {
    let mut r = Rot3::new(5.0, Bivec3::new(3.0, 2.0, 1.0));
    r -= Rot3::new(2.0, Bivec3::new(1.0, 1.0, 1.0));
    assert_eq!(r.s, 3.0);
    assert_eq!(r.bv.xy, 2.0);
    assert_eq!(r.bv.xz, 1.0);
    assert_eq!(r.bv.yz, 0.0);
}

#[test]
fn test_mul_scalar() {
    let r = Rot3::new(2.0, Bivec3::new(1.0, 2.0, 3.0));
    let scaled = r * 3.0;
    assert_eq!(scaled.s, 6.0);
    assert_eq!(scaled.bv.xy, 3.0);
    assert_eq!(scaled.bv.xz, 6.0);
    assert_eq!(scaled.bv.yz, 9.0);

    let scaled2 = 3.0 * r;
    assert_eq!(scaled2.s, 6.0);
    assert_eq!(scaled2.bv.xy, 3.0);
}

#[test]
fn test_mul_assign_scalar() {
    let mut r = Rot3::new(2.0, Bivec3::new(1.0, 2.0, 3.0));
    r *= 3.0;
    assert_eq!(r.s, 6.0);
    assert_eq!(r.bv.xy, 3.0);
    assert_eq!(r.bv.xz, 6.0);
    assert_eq!(r.bv.yz, 9.0);
}

#[test]
fn test_div_scalar() {
    let r = Rot3::new(6.0, Bivec3::new(3.0, 6.0, 9.0));
    let scaled = r / 3.0;
    assert_eq!(scaled.s, 2.0);
    assert_eq!(scaled.bv.xy, 1.0);
    assert_eq!(scaled.bv.xz, 2.0);
    assert_eq!(scaled.bv.yz, 3.0);
}

#[test]
fn test_div_assign_scalar() {
    let mut r = Rot3::new(6.0, Bivec3::new(3.0, 6.0, 9.0));
    r /= 3.0;
    assert_eq!(r.s, 2.0);
    assert_eq!(r.bv.xy, 1.0);
    assert_eq!(r.bv.xz, 2.0);
    assert_eq!(r.bv.yz, 3.0);
}

#[test]
fn test_dot() {
    let r1 = Rot3::new(1.0, Bivec3::new(2.0, 3.0, 4.0));
    let r2 = Rot3::new(1.0, Bivec3::new(2.0, 3.0, 4.0));
    // dot = 1 + 4 + 9 + 16 = 30
    assert_eq!(r1.dot(r2), 30.0);

    let r3 = Rot3::new(1.0, Bivec3::new(0.0, 0.0, 0.0));
    let r4 = Rot3::new(0.0, Bivec3::new(1.0, 0.0, 0.0));
    assert_eq!(r3.dot(r4), 0.0);
}

#[test]
fn test_rotate_by_identity() {
    let b = Rot3::from_angle_plane(std::f32::consts::PI / 3.0, Bivec3::new(1.0, 0.0, 0.0));
    let mut r = b;
    r.rotate_by(Rot3::identity());
    assert!(approx_eq(r.s, b.s));
    assert!(approx_eq(r.bv.xy, b.bv.xy));
    assert!(approx_eq(r.bv.xz, b.bv.xz));
    assert!(approx_eq(r.bv.yz, b.bv.yz));
}

#[test]
fn test_rotated_by_identity() {
    let b = Rot3::from_angle_plane(std::f32::consts::PI / 3.0, Bivec3::new(1.0, 0.0, 0.0));
    let r = b.rotated_by(Rot3::identity());
    assert!(approx_eq(r.s, b.s));
    assert!(approx_eq(r.bv.xy, b.bv.xy));
    assert!(approx_eq(r.bv.xz, b.bv.xz));
    assert!(approx_eq(r.bv.yz, b.bv.yz));
}

#[test]
fn test_rotated_by_preserves_magnitude() {
    let b = Rot3::from_angle_plane(std::f32::consts::PI / 4.0, Bivec3::new(1.0, 0.0, 0.0)).normalized();
    let a = Rot3::from_angle_plane(std::f32::consts::PI / 3.0, Bivec3::new(0.0, 1.0, 0.0)).normalized();
    let r = b.rotated_by(a);
    assert!(approx_eq(r.mag(), b.mag()));
}

#[test]
fn test_into_matrix_identity() {
    let r = Rot3::identity();
    let m = r.into_matrix();
    // Identity matrix in column-major: col0=(1,0,0), col1=(0,1,0), col2=(0,0,1)
    assert!(approx_eq(m.cols[0].x, 1.0));
    assert!(approx_eq(m.cols[0].y, 0.0));
    assert!(approx_eq(m.cols[0].z, 0.0));
    assert!(approx_eq(m.cols[1].x, 0.0));
    assert!(approx_eq(m.cols[1].y, 1.0));
    assert!(approx_eq(m.cols[1].z, 0.0));
    assert!(approx_eq(m.cols[2].x, 0.0));
    assert!(approx_eq(m.cols[2].y, 0.0));
    assert!(approx_eq(m.cols[2].z, 1.0));
}

#[test]
fn test_into_matrix_90_in_xy_plane() {
    let plane = Bivec3::new(1.0, 0.0, 0.0);
    let r = Rot3::from_angle_plane(std::f32::consts::PI / 2.0, plane);
    let m = r.into_matrix();
    // (1,0,0) → col0 of matrix applied to x-axis
    let col0 = m.cols[0];
    assert!(approx_eq(col0.x, 0.0));
    assert!(approx_eq(col0.y, 1.0));
    assert!(approx_eq(col0.z, 0.0));
}
