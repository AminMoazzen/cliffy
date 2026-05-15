use cliffy::*;

#[test]
fn test_construction() {
    let col0 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let col1 = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let col2 = Vec4::new(9.0, 10.0, 11.0, 12.0);
    let col3 = Vec4::new(13.0, 14.0, 15.0, 16.0);
    let m = Mat4::new(col0, col1, col2, col3);
    assert_eq!(m.cols[0], col0);
    assert_eq!(m.cols[1], col1);
    assert_eq!(m.cols[2], col2);
    assert_eq!(m.cols[3], col3);
}

#[test]
fn test_identity() {
    let m = Mat4::identity();
    assert_eq!(m.cols[0], Vec4::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(m.cols[1], Vec4::new(0.0, 1.0, 0.0, 0.0));
    assert_eq!(m.cols[2], Vec4::new(0.0, 0.0, 1.0, 0.0));
    assert_eq!(m.cols[3], Vec4::new(0.0, 0.0, 0.0, 1.0));
}

#[test]
fn test_zero() {
    let m = Mat4::zero();
    for c in 0..4 {
        assert_eq!(m.cols[c], Vec4::zero());
    }
}

#[test]
fn test_transpose() {
    let m = Mat4::new(
        Vec4::new(1.0,  2.0,  3.0,  4.0),
        Vec4::new(5.0,  6.0,  7.0,  8.0),
        Vec4::new(9.0,  10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    let t = m.transpose();
    assert_eq!(t.cols[0], Vec4::new(1.0, 5.0, 9.0, 13.0));
    assert_eq!(t.cols[1], Vec4::new(2.0, 6.0, 10.0, 14.0));
    assert_eq!(t.cols[2], Vec4::new(3.0, 7.0, 11.0, 15.0));
    assert_eq!(t.cols[3], Vec4::new(4.0, 8.0, 12.0, 16.0));
}

#[test]
fn test_transpose_identity() {
    assert_eq!(Mat4::identity().transpose(), Mat4::identity());
}

#[test]
fn test_transpose_twice() {
    let m = Mat4::new(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(m.transpose().transpose(), m);
}

#[test]
fn test_det_identity() {
    assert_eq!(Mat4::identity().det(), 1.0);
}

#[test]
fn test_det_zero_matrix() {
    assert_eq!(Mat4::zero().det(), 0.0);
}

#[test]
fn test_det_diagonal() {
    // Diagonal 2,3,4,5 → det = 2*3*4*5 = 120
    let m = Mat4::new(
        Vec4::new(2.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 3.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 4.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 5.0),
    );
    assert!((m.det() - 120.0).abs() < 1e-4);
}

#[test]
fn test_det_singular() {
    // Last two columns identical → singular
    let m = Mat4::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
    );
    assert_eq!(m.det(), 0.0);
}

#[test]
fn test_det_transpose_equal() {
    let m = Mat4::new(
        Vec4::new(1.0, 2.0, 0.0, 0.0),
        Vec4::new(0.0, 3.0, 4.0, 0.0),
        Vec4::new(0.0, 0.0, 5.0, 6.0),
        Vec4::new(0.0, 0.0, 0.0, 7.0),
    );
    assert!((m.det() - m.transpose().det()).abs() < 1e-4);
}

#[test]
fn test_inverse_identity() {
    let inv = Mat4::identity().inverse().unwrap();
    assert_eq!(inv, Mat4::identity());
}

#[test]
fn test_inverse_singular() {
    let m = Mat4::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 1.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
        Vec4::new(0.0, 0.0, 1.0, 0.0),
    );
    assert!(m.inverse().is_none());
}

#[test]
fn test_inverse_diagonal() {
    let m = Mat4::new(
        Vec4::new(2.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 4.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 5.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 10.0),
    );
    let inv = m.inverse().unwrap();
    assert!((inv.cols[0].x - 0.5).abs() < 1e-6);
    assert!((inv.cols[1].y - 0.25).abs() < 1e-6);
    assert!((inv.cols[2].z - 0.2).abs() < 1e-6);
    assert!((inv.cols[3].w - 0.1).abs() < 1e-6);
}

#[test]
fn test_inverse_roundtrip() {
    let m = Mat4::new(
        Vec4::new(1.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 2.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 3.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 4.0),
    );
    let inv = m.inverse().unwrap();
    let product = m * inv;
    for c in 0..4 {
        for r in 0..4 {
            let expected = if r == c { 1.0_f32 } else { 0.0_f32 };
            let actual = match r {
                0 => product.cols[c].x,
                1 => product.cols[c].y,
                2 => product.cols[c].z,
                _ => product.cols[c].w,
            };
            assert!((actual - expected).abs() < 1e-5, "product[{}][{}] = {}, expected {}", c, r, actual, expected);
        }
    }
}

#[test]
fn test_mul_vec_identity() {
    let v = Vec4::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Mat4::identity() * v, v);
}

#[test]
fn test_mul_vec_scale() {
    let m = Mat4::new(
        Vec4::new(2.0, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 3.0, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 4.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, 5.0),
    );
    let v = Vec4::new(1.0, 1.0, 1.0, 1.0);
    assert_eq!(m * v, Vec4::new(2.0, 3.0, 4.0, 5.0));
}

#[test]
fn test_mul_mat_identity() {
    let m = Mat4::new(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(m * Mat4::identity(), m);
    assert_eq!(Mat4::identity() * m, m);
}

#[test]
fn test_mul_scalar() {
    let m = Mat4::identity();
    let scaled = m * 4.0;
    assert_eq!(scaled.cols[0], Vec4::new(4.0, 0.0, 0.0, 0.0));
    assert_eq!(scaled.cols[1], Vec4::new(0.0, 4.0, 0.0, 0.0));
    assert_eq!(scaled.cols[2], Vec4::new(0.0, 0.0, 4.0, 0.0));
    assert_eq!(scaled.cols[3], Vec4::new(0.0, 0.0, 0.0, 4.0));
}

#[test]
fn test_mul_scalar_commutative() {
    let m = Mat4::new(
        Vec4::new(1.0, 2.0, 3.0, 4.0),
        Vec4::new(5.0, 6.0, 7.0, 8.0),
        Vec4::new(9.0, 10.0, 11.0, 12.0),
        Vec4::new(13.0, 14.0, 15.0, 16.0),
    );
    assert_eq!(m * 2.0, 2.0 * m);
}

#[test]
fn test_add() {
    let a = Mat4::identity();
    let c = a + a;
    assert_eq!(c.cols[0], Vec4::new(2.0, 0.0, 0.0, 0.0));
    assert_eq!(c.cols[1], Vec4::new(0.0, 2.0, 0.0, 0.0));
    assert_eq!(c.cols[2], Vec4::new(0.0, 0.0, 2.0, 0.0));
    assert_eq!(c.cols[3], Vec4::new(0.0, 0.0, 0.0, 2.0));
}

#[test]
fn test_add_assign() {
    let mut a = Mat4::identity();
    a += Mat4::identity();
    assert_eq!(a.cols[0], Vec4::new(2.0, 0.0, 0.0, 0.0));
}

#[test]
fn test_sub() {
    let a = Mat4::identity();
    assert_eq!(a - a, Mat4::zero());
}

#[test]
fn test_sub_assign() {
    let mut a = Mat4::identity();
    a -= Mat4::identity();
    assert_eq!(a, Mat4::zero());
}

#[test]
fn test_neg() {
    let m = Mat4::identity();
    let n = -m;
    assert_eq!(n.cols[0], Vec4::new(-1.0, 0.0, 0.0, 0.0));
    assert_eq!(n.cols[1], Vec4::new(0.0, -1.0, 0.0, 0.0));
    assert_eq!(n.cols[2], Vec4::new(0.0, 0.0, -1.0, 0.0));
    assert_eq!(n.cols[3], Vec4::new(0.0, 0.0, 0.0, -1.0));
}

#[test]
fn test_index() {
    let col0 = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let col1 = Vec4::new(5.0, 6.0, 7.0, 8.0);
    let col2 = Vec4::new(9.0, 10.0, 11.0, 12.0);
    let col3 = Vec4::new(13.0, 14.0, 15.0, 16.0);
    let m = Mat4::new(col0, col1, col2, col3);
    assert_eq!(m[0], col0);
    assert_eq!(m[1], col1);
    assert_eq!(m[2], col2);
    assert_eq!(m[3], col3);
}

#[test]
fn test_index_mut() {
    let mut m = Mat4::zero();
    m[0] = Vec4::new(1.0, 0.0, 0.0, 0.0);
    m[1] = Vec4::new(0.0, 1.0, 0.0, 0.0);
    m[2] = Vec4::new(0.0, 0.0, 1.0, 0.0);
    m[3] = Vec4::new(0.0, 0.0, 0.0, 1.0);
    assert_eq!(m, Mat4::identity());
}

#[test]
#[should_panic]
fn test_index_out_of_range() {
    let m = Mat4::identity();
    let _ = m[4];
}

#[test]
#[should_panic]
fn test_index_mut_out_of_range() {
    let mut m = Mat4::identity();
    m[4] = Vec4::zero();
}
