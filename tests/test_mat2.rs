use cliffy::*;

#[test]
fn test_construction() {
    let col0 = Vec2::new(1.0, 2.0);
    let col1 = Vec2::new(3.0, 4.0);
    let m = Mat2::new(col0, col1);
    assert_eq!(m.cols[0], col0);
    assert_eq!(m.cols[1], col1);
}

#[test]
fn test_identity() {
    let m = Mat2::identity();
    assert_eq!(m.cols[0], Vec2::new(1.0, 0.0));
    assert_eq!(m.cols[1], Vec2::new(0.0, 1.0));
}

#[test]
fn test_zero() {
    let m = Mat2::zero();
    assert_eq!(m.cols[0], Vec2::zero());
    assert_eq!(m.cols[1], Vec2::zero());
}

#[test]
fn test_transpose() {
    let m = Mat2::new(
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 4.0),
    );
    let t = m.transpose();
    assert_eq!(t.cols[0], Vec2::new(1.0, 3.0));
    assert_eq!(t.cols[1], Vec2::new(2.0, 4.0));
}

#[test]
fn test_transpose_identity() {
    let m = Mat2::identity();
    assert_eq!(m.transpose(), m);
}

#[test]
fn test_transpose_twice() {
    let m = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    assert_eq!(m.transpose().transpose(), m);
}

#[test]
fn test_det() {
    let m = Mat2::new(
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 4.0),
    );
    // det = 1*4 - 3*2 = -2
    assert_eq!(m.det(), -2.0);
}

#[test]
fn test_det_identity() {
    assert_eq!(Mat2::identity().det(), 1.0);
}

#[test]
fn test_det_zero() {
    assert_eq!(Mat2::zero().det(), 0.0);
}

#[test]
fn test_inverse() {
    let m = Mat2::new(
        Vec2::new(1.0, 2.0),
        Vec2::new(3.0, 4.0),
    );
    let inv = m.inverse().unwrap();
    // inv of [[1,3],[2,4]] = (1/-2) * [[4,-3],[-2,1]]
    // col0 = (4/-2, -2/-2) = (-2, 1)
    // col1 = (-3/-2, 1/-2) = (1.5, -0.5)
    assert_eq!(inv.cols[0], Vec2::new(-2.0, 1.0));
    assert_eq!(inv.cols[1], Vec2::new(1.5, -0.5));
}

#[test]
fn test_inverse_identity() {
    let inv = Mat2::identity().inverse().unwrap();
    assert_eq!(inv, Mat2::identity());
}

#[test]
fn test_inverse_singular() {
    // Columns are parallel → singular
    let m = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(2.0, 4.0));
    assert!(m.inverse().is_none());
}

#[test]
fn test_inverse_roundtrip() {
    let m = Mat2::new(
        Vec2::new(2.0, 0.0),
        Vec2::new(0.0, 3.0),
    );
    let inv = m.inverse().unwrap();
    let product = m * inv;
    // Should be identity (within float precision)
    assert!((product.cols[0].x - 1.0).abs() < 1e-6);
    assert!((product.cols[0].y).abs() < 1e-6);
    assert!((product.cols[1].x).abs() < 1e-6);
    assert!((product.cols[1].y - 1.0).abs() < 1e-6);
}

#[test]
fn test_mul_vec() {
    let m = Mat2::identity();
    let v = Vec2::new(3.0, 5.0);
    assert_eq!(m * v, v);
}

#[test]
fn test_mul_vec_nontrivial() {
    // Rotation by 90 degrees: col0=(0,1), col1=(-1,0)
    // Maps (1,0) -> (0,1), (0,1) -> (-1,0)
    let m = Mat2::new(Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.0));
    let v = Vec2::new(1.0, 0.0);
    assert_eq!(m * v, Vec2::new(0.0, 1.0));
}

#[test]
fn test_mul_mat() {
    let a = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    let b = Mat2::identity();
    assert_eq!(a * b, a);
    assert_eq!(b * a, a);
}

#[test]
fn test_mul_mat_nontrivial() {
    let a = Mat2::new(Vec2::new(1.0, 3.0), Vec2::new(2.0, 4.0));
    let b = Mat2::new(Vec2::new(5.0, 7.0), Vec2::new(6.0, 8.0));
    // (A*B).col0 = A * b.col0 = A*(5,7)
    //   x = 1*5 + 2*7 = 19
    //   y = 3*5 + 4*7 = 43
    // (A*B).col1 = A * b.col1 = A*(6,8)
    //   x = 1*6 + 2*8 = 22
    //   y = 3*6 + 4*8 = 50
    let c = a * b;
    assert_eq!(c.cols[0], Vec2::new(19.0, 43.0));
    assert_eq!(c.cols[1], Vec2::new(22.0, 50.0));
}

#[test]
fn test_mul_scalar() {
    let m = Mat2::identity();
    let scaled = m * 3.0;
    assert_eq!(scaled.cols[0], Vec2::new(3.0, 0.0));
    assert_eq!(scaled.cols[1], Vec2::new(0.0, 3.0));
}

#[test]
fn test_mul_scalar_commutative() {
    let m = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    assert_eq!(m * 2.0, 2.0 * m);
}

#[test]
fn test_add() {
    let a = Mat2::identity();
    let b = Mat2::identity();
    let c = a + b;
    assert_eq!(c.cols[0], Vec2::new(2.0, 0.0));
    assert_eq!(c.cols[1], Vec2::new(0.0, 2.0));
}

#[test]
fn test_add_assign() {
    let mut a = Mat2::identity();
    a += Mat2::identity();
    assert_eq!(a.cols[0], Vec2::new(2.0, 0.0));
    assert_eq!(a.cols[1], Vec2::new(0.0, 2.0));
}

#[test]
fn test_sub() {
    let a = Mat2::identity();
    let b = Mat2::identity();
    assert_eq!(a - b, Mat2::zero());
}

#[test]
fn test_sub_assign() {
    let mut a = Mat2::identity();
    a -= Mat2::identity();
    assert_eq!(a, Mat2::zero());
}

#[test]
fn test_neg() {
    let m = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    let n = -m;
    assert_eq!(n.cols[0], Vec2::new(-1.0, -2.0));
    assert_eq!(n.cols[1], Vec2::new(-3.0, -4.0));
}

#[test]
fn test_index() {
    let m = Mat2::new(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    assert_eq!(m[0], Vec2::new(1.0, 2.0));
    assert_eq!(m[1], Vec2::new(3.0, 4.0));
}

#[test]
fn test_index_mut() {
    let mut m = Mat2::zero();
    m[0] = Vec2::new(1.0, 0.0);
    m[1] = Vec2::new(0.0, 1.0);
    assert_eq!(m, Mat2::identity());
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let m = Mat2::identity();
    let _ = m[2];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut m = Mat2::identity();
    m[2] = Vec2::zero();
}

#[test]
fn test_identity_mul_vector_is_identity() {
    let v = Vec2::new(7.0, -3.0);
    assert_eq!(Mat2::identity() * v, v);
}

#[test]
fn test_det_transpose_equal() {
    let m = Mat2::new(Vec2::new(2.0, 5.0), Vec2::new(3.0, 7.0));
    assert_eq!(m.det(), m.transpose().det());
}
