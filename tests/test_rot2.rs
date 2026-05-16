use cliffy::*;

const EPS: f32 = 1e-6;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn test_constructions() {
    let r = Rot2::new(0.5, Bivec2::new(-0.5));
    assert_eq!(r.s, 0.5);
    assert_eq!(r.bv, Bivec2::new(-0.5));

    let id = Rot2::identity();
    assert_eq!(id.s, 1.0);
    assert_eq!(id.bv, Bivec2::zero());
}

#[test]
fn test_from_angle() {
    let angle = std::f32::consts::PI / 2.0;
    let half = angle / 2.0;
    let (sin, cos) = half.sin_cos();
    let r = Rot2::from_angle(angle);
    assert!(approx_eq(r.s, cos));
    assert!(approx_eq(r.bv.xy, -sin));
}

#[test]
fn test_from_angle_zero() {
    let r = Rot2::from_angle(0.0);
    assert!(approx_eq(r.s, 1.0));
    assert!(approx_eq(r.bv.xy, 0.0));
}

#[test]
fn test_identity_rotate_vec() {
    let r = Rot2::identity();
    let v = Vec2::new(3.0, 4.0);
    let rotated = r * v;
    assert!(approx_eq(rotated.x, v.x));
    assert!(approx_eq(rotated.y, v.y));
}

#[test]
fn test_rotate_90_degrees() {
    let r = Rot2::from_angle(std::f32::consts::PI / 2.0);
    let rotated = r * Vec2::right();
    assert!(approx_eq(rotated.x, 0.0));
    assert!(approx_eq(rotated.y, 1.0));
}

#[test]
fn test_rotate_180_degrees() {
    let r = Rot2::from_angle(std::f32::consts::PI);
    let rotated = r * Vec2::right();
    assert!(approx_eq(rotated.x, -1.0));
    assert!(approx_eq(rotated.y, 0.0));
}

#[test]
fn test_rotate_360_degrees() {
    let r = Rot2::from_angle(2.0 * std::f32::consts::PI);
    let v = Vec2::new(3.0, 4.0);
    let rotated = r * v;
    assert!(approx_eq(rotated.x, v.x));
    assert!(approx_eq(rotated.y, v.y));
}

#[test]
fn test_mag_identity() {
    let r = Rot2::identity();
    assert!(approx_eq(r.mag(), 1.0));
    assert!(approx_eq(r.mag_sq(), 1.0));
}

#[test]
fn test_mag() {
    let r = Rot2::new(3.0, Bivec2::new(4.0));
    assert!(approx_eq(r.mag_sq(), 25.0));
    assert!(approx_eq(r.mag(), 5.0));
}

#[test]
fn test_normalize() {
    let mut r = Rot2::new(3.0, Bivec2::new(4.0));
    r.normalize();
    assert!(approx_eq(r.mag(), 1.0));
}

#[test]
fn test_normalized() {
    let r = Rot2::new(3.0, Bivec2::new(4.0));
    let rn = r.normalized();
    assert!(approx_eq(rn.mag(), 1.0));
    assert!(approx_eq(rn.s, 3.0 / 5.0));
    assert!(approx_eq(rn.bv.xy, 4.0 / 5.0));
}

#[test]
fn test_reverse() {
    let r = Rot2::from_angle(std::f32::consts::PI / 3.0);
    let rev = r.reversed();
    assert_eq!(rev.s, r.s);
    assert_eq!(rev.bv.xy, -r.bv.xy);
}

#[test]
fn test_reverse_in_place() {
    let r = Rot2::new(1.0, Bivec2::new(2.0));
    let mut r2 = r;
    r2.reverse();
    assert_eq!(r2.s, 1.0);
    assert_eq!(r2.bv.xy, -2.0);
}

#[test]
fn test_reverse_undoes_rotation() {
    let r = Rot2::from_angle(std::f32::consts::PI / 3.0).normalized();
    let v = Vec2::new(3.0, 4.0);
    let forward = r * v;
    let back = r.reversed() * forward;
    assert!(approx_eq(back.x, v.x));
    assert!(approx_eq(back.y, v.y));
}

#[test]
fn test_mul_rotor_by_reverse_is_identity() {
    let r = Rot2::from_angle(std::f32::consts::PI / 4.0).normalized();
    let product = r * r.reversed();
    assert!(approx_eq(product.s, 1.0));
    assert!(approx_eq(product.bv.xy, 0.0));
}

#[test]
fn test_mul_two_rotations() {
    let r1 = Rot2::from_angle(std::f32::consts::PI / 2.0);
    let r2 = Rot2::from_angle(std::f32::consts::PI / 2.0);
    let combined = r1 * r2;
    let v = Vec2::right();
    let rotated = combined * v;
    assert!(approx_eq(rotated.x, -1.0));
    assert!(approx_eq(rotated.y, 0.0));
}

#[test]
fn test_add() {
    let r1 = Rot2::new(1.0, Bivec2::new(2.0));
    let r2 = Rot2::new(3.0, Bivec2::new(-1.0));
    let sum = r1 + r2;
    assert_eq!(sum.s, 4.0);
    assert_eq!(sum.bv.xy, 1.0);
}

#[test]
fn test_add_assign() {
    let mut r = Rot2::new(1.0, Bivec2::new(2.0));
    r += Rot2::new(3.0, Bivec2::new(-1.0));
    assert_eq!(r.s, 4.0);
    assert_eq!(r.bv.xy, 1.0);
}

#[test]
fn test_sub() {
    let r1 = Rot2::new(5.0, Bivec2::new(3.0));
    let r2 = Rot2::new(2.0, Bivec2::new(1.0));
    let diff = r1 - r2;
    assert_eq!(diff.s, 3.0);
    assert_eq!(diff.bv.xy, 2.0);
}

#[test]
fn test_sub_assign() {
    let mut r = Rot2::new(5.0, Bivec2::new(3.0));
    r -= Rot2::new(2.0, Bivec2::new(1.0));
    assert_eq!(r.s, 3.0);
    assert_eq!(r.bv.xy, 2.0);
}

#[test]
fn test_mul_scalar() {
    let r = Rot2::new(2.0, Bivec2::new(3.0));
    let scaled = r * 2.0;
    assert_eq!(scaled.s, 4.0);
    assert_eq!(scaled.bv.xy, 6.0);

    let scaled2 = 2.0 * r;
    assert_eq!(scaled2.s, 4.0);
    assert_eq!(scaled2.bv.xy, 6.0);
}

#[test]
fn test_mul_assign_scalar() {
    let mut r = Rot2::new(2.0, Bivec2::new(3.0));
    r *= 2.0;
    assert_eq!(r.s, 4.0);
    assert_eq!(r.bv.xy, 6.0);
}

#[test]
fn test_div_scalar() {
    let r = Rot2::new(4.0, Bivec2::new(6.0));
    let scaled = r / 2.0;
    assert_eq!(scaled.s, 2.0);
    assert_eq!(scaled.bv.xy, 3.0);
}

#[test]
fn test_div_assign_scalar() {
    let mut r = Rot2::new(4.0, Bivec2::new(6.0));
    r /= 2.0;
    assert_eq!(r.s, 2.0);
    assert_eq!(r.bv.xy, 3.0);
}

#[test]
fn test_dot() {
    let r1 = Rot2::new(1.0, Bivec2::new(0.0));
    let r2 = Rot2::new(0.0, Bivec2::new(1.0));
    assert_eq!(r1.dot(r2), 0.0);

    let r3 = Rot2::new(3.0, Bivec2::new(4.0));
    assert_eq!(r3.dot(r3), 25.0);
}

#[test]
fn test_rotate_vec_direct() {
    let r = Rot2::from_angle(std::f32::consts::PI / 2.0);
    let mut v = Vec2::right();
    r.rotate_vec(&mut v);
    assert!(approx_eq(v.x, 0.0));
    assert!(approx_eq(v.y, 1.0));
}

#[test]
fn test_into_matrix_identity() {
    let r = Rot2::identity();
    let m = r.into_matrix();
    assert!(approx_eq(m.cols[0].x, 1.0));
    assert!(approx_eq(m.cols[0].y, 0.0));
    assert!(approx_eq(m.cols[1].x, 0.0));
    assert!(approx_eq(m.cols[1].y, 1.0));
}

#[test]
fn test_into_matrix_90_degrees() {
    let r = Rot2::from_angle(std::f32::consts::PI / 2.0);
    let m = r.into_matrix();
    // Column-major 90° CCW rotation: col0=(0,1), col1=(-1,0)
    assert!(approx_eq(m.cols[0].x, 0.0));
    assert!(approx_eq(m.cols[0].y, 1.0));
    assert!(approx_eq(m.cols[1].x, -1.0));
    assert!(approx_eq(m.cols[1].y, 0.0));
}

#[test]
fn test_rotate_by_identity() {
    let b = Rot2::from_angle(std::f32::consts::PI / 3.0);
    let mut r = b;
    r.rotate_by(Rot2::identity());
    assert!(approx_eq(r.s, b.s));
    assert!(approx_eq(r.bv.xy, b.bv.xy));
}

#[test]
fn test_rotated_by_identity() {
    let b = Rot2::from_angle(std::f32::consts::PI / 3.0);
    let r = b.rotated_by(Rot2::identity());
    assert!(approx_eq(r.s, b.s));
    assert!(approx_eq(r.bv.xy, b.bv.xy));
}

#[test]
fn test_rotated_by_normalized_preserves_rotor() {
    // In 2D, a * b * ~a = b * |a|^2, so for normalized a it equals b exactly
    let b = Rot2::new(3.0, Bivec2::new(4.0));
    let a = Rot2::from_angle(std::f32::consts::PI / 5.0).normalized();
    let r = b.rotated_by(a);
    assert!(approx_eq(r.s, b.s));
    assert!(approx_eq(r.bv.xy, b.bv.xy));
}

#[test]
fn test_rotated_by_scales_by_mag_sq() {
    // a * b * ~a = b * |a|^2
    let b = Rot2::new(1.0, Bivec2::new(2.0));
    let a = Rot2::new(3.0, Bivec2::new(4.0)); // |a|^2 = 25
    let r = b.rotated_by(a);
    assert!(approx_eq(r.s, b.s * 25.0));
    assert!(approx_eq(r.bv.xy, b.bv.xy * 25.0));
}
