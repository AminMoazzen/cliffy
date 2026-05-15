use cliffy::*;

#[test]
fn test_construction() {
    let col0 = Vec3::new(1.0, 2.0, 3.0);
    let col1 = Vec3::new(4.0, 5.0, 6.0);
    let col2 = Vec3::new(7.0, 8.0, 9.0);
    let m = Mat3::new(col0, col1, col2);
    assert_eq!(m.cols[0], col0);
    assert_eq!(m.cols[1], col1);
    assert_eq!(m.cols[2], col2);
}

#[test]
fn test_identity() {
    let m = Mat3::identity();
    assert_eq!(m.cols[0], Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(m.cols[1], Vec3::new(0.0, 1.0, 0.0));
    assert_eq!(m.cols[2], Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn test_zero() {
    let m = Mat3::zero();
    assert_eq!(m.cols[0], Vec3::zero());
    assert_eq!(m.cols[1], Vec3::zero());
    assert_eq!(m.cols[2], Vec3::zero());
}

#[test]
fn test_transpose() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    let t = m.transpose();
    assert_eq!(t.cols[0], Vec3::new(1.0, 4.0, 7.0));
    assert_eq!(t.cols[1], Vec3::new(2.0, 5.0, 8.0));
    assert_eq!(t.cols[2], Vec3::new(3.0, 6.0, 9.0));
}

#[test]
fn test_transpose_identity() {
    assert_eq!(Mat3::identity().transpose(), Mat3::identity());
}

#[test]
fn test_transpose_twice() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    assert_eq!(m.transpose().transpose(), m);
}

#[test]
fn test_det_identity() {
    assert_eq!(Mat3::identity().det(), 1.0);
}

#[test]
fn test_det_zero() {
    assert_eq!(Mat3::zero().det(), 0.0);
}

#[test]
fn test_det() {
    // Diagonal matrix: det = 2*3*4 = 24
    let m = Mat3::new(
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
    );
    assert_eq!(m.det(), 24.0);
}

#[test]
fn test_det_singular() {
    // Row 2 = 2 * row 0 → singular
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(2.0, 4.0, 6.0),
    );
    assert_eq!(m.det(), 0.0);
}

#[test]
fn test_det_transpose_equal() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(0.0, 5.0, 6.0),
        Vec3::new(0.0, 0.0, 9.0),
    );
    assert_eq!(m.det(), m.transpose().det());
}

#[test]
fn test_inverse_identity() {
    let inv = Mat3::identity().inverse().unwrap();
    assert_eq!(inv, Mat3::identity());
}

#[test]
fn test_inverse_singular() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(2.0, 4.0, 6.0),
    );
    assert!(m.inverse().is_none());
}

#[test]
fn test_inverse_diagonal() {
    let m = Mat3::new(
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        Vec3::new(0.0, 0.0, 5.0),
    );
    let inv = m.inverse().unwrap();
    assert!((inv.cols[0].x - 0.5).abs() < 1e-6);
    assert!((inv.cols[1].y - 0.25).abs() < 1e-6);
    assert!((inv.cols[2].z - 0.2).abs() < 1e-6);
}

#[test]
fn test_inverse_roundtrip() {
    let m = Mat3::new(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, 0.0, 3.0),
    );
    let inv = m.inverse().unwrap();
    let product = m * inv;
    for c in 0..3 {
        for r in 0..3 {
            let expected = if r == c { 1.0 } else { 0.0 };
            let actual = match r {
                0 => product.cols[c].x,
                1 => product.cols[c].y,
                _ => product.cols[c].z,
            };
            assert!((actual - expected).abs() < 1e-5, "product[{}][{}] = {}, expected {}", c, r, actual, expected);
        }
    }
}

#[test]
fn test_mul_vec_identity() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(Mat3::identity() * v, v);
}

#[test]
fn test_mul_vec_scale() {
    let m = Mat3::new(
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 3.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
    );
    let v = Vec3::new(1.0, 1.0, 1.0);
    assert_eq!(m * v, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn test_mul_mat_identity() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    assert_eq!(m * Mat3::identity(), m);
    assert_eq!(Mat3::identity() * m, m);
}

#[test]
fn test_mul_mat_nontrivial() {
    let a = Mat3::new(
        Vec3::new(1.0, 4.0, 7.0),
        Vec3::new(2.0, 5.0, 8.0),
        Vec3::new(3.0, 6.0, 9.0),
    );
    let b = Mat3::new(
        Vec3::new(9.0, 6.0, 3.0),
        Vec3::new(8.0, 5.0, 2.0),
        Vec3::new(7.0, 4.0, 1.0),
    );
    // (A*B).col0 = A * (9,6,3)
    // x = 1*9 + 2*6 + 3*3 = 9+12+9 = 30
    // y = 4*9 + 5*6 + 6*3 = 36+30+18 = 84
    // z = 7*9 + 8*6 + 9*3 = 63+48+27 = 138
    let c = a * b;
    assert_eq!(c.cols[0], Vec3::new(30.0, 84.0, 138.0));
}

#[test]
fn test_mul_scalar() {
    let m = Mat3::identity();
    let scaled = m * 5.0;
    assert_eq!(scaled.cols[0], Vec3::new(5.0, 0.0, 0.0));
    assert_eq!(scaled.cols[1], Vec3::new(0.0, 5.0, 0.0));
    assert_eq!(scaled.cols[2], Vec3::new(0.0, 0.0, 5.0));
}

#[test]
fn test_mul_scalar_commutative() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    assert_eq!(m * 3.0, 3.0 * m);
}

#[test]
fn test_add() {
    let a = Mat3::identity();
    let c = a + a;
    assert_eq!(c.cols[0], Vec3::new(2.0, 0.0, 0.0));
    assert_eq!(c.cols[1], Vec3::new(0.0, 2.0, 0.0));
    assert_eq!(c.cols[2], Vec3::new(0.0, 0.0, 2.0));
}

#[test]
fn test_add_assign() {
    let mut a = Mat3::identity();
    a += Mat3::identity();
    assert_eq!(a.cols[0], Vec3::new(2.0, 0.0, 0.0));
}

#[test]
fn test_sub() {
    let a = Mat3::identity();
    assert_eq!(a - a, Mat3::zero());
}

#[test]
fn test_sub_assign() {
    let mut a = Mat3::identity();
    a -= Mat3::identity();
    assert_eq!(a, Mat3::zero());
}

#[test]
fn test_neg() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    let n = -m;
    assert_eq!(n.cols[0], Vec3::new(-1.0, -2.0, -3.0));
    assert_eq!(n.cols[1], Vec3::new(-4.0, -5.0, -6.0));
    assert_eq!(n.cols[2], Vec3::new(-7.0, -8.0, -9.0));
}

#[test]
fn test_index() {
    let m = Mat3::new(
        Vec3::new(1.0, 2.0, 3.0),
        Vec3::new(4.0, 5.0, 6.0),
        Vec3::new(7.0, 8.0, 9.0),
    );
    assert_eq!(m[0], Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(m[1], Vec3::new(4.0, 5.0, 6.0));
    assert_eq!(m[2], Vec3::new(7.0, 8.0, 9.0));
}

#[test]
fn test_index_mut() {
    let mut m = Mat3::zero();
    m[0] = Vec3::new(1.0, 0.0, 0.0);
    m[1] = Vec3::new(0.0, 1.0, 0.0);
    m[2] = Vec3::new(0.0, 0.0, 1.0);
    assert_eq!(m, Mat3::identity());
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let m = Mat3::identity();
    let _ = m[3];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut m = Mat3::identity();
    m[3] = Vec3::zero();
}
