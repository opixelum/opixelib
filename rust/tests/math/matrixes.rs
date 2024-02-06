use opixelib::math::matrixes::*;

#[test]
fn test_new_matrixes() {
    let matrix_1d: Matrix<u8> = Matrix::new(vec![3]);
    assert_eq!(matrix_1d.data, vec![0, 0, 0]);
    assert_eq!(matrix_1d.shape, vec![3]);

    let matrix_2d: Matrix<u8> = Matrix::new(vec![2, 3]);
    assert_eq!(matrix_2d.data, vec![0, 0, 0, 0, 0, 0]);
    assert_eq!(matrix_2d.shape, vec![2, 3]);

    let matrix_3d: Matrix<u8> = Matrix::new(vec![2, 2, 2]);
    assert_eq!(matrix_3d.data, vec![0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(matrix_3d.shape, vec![2, 2, 2]);
}

#[test]
fn test_calculate_index_matrixes() {
    let matrix_1d = Matrix::<u8>::new(vec![3]);
    assert_eq!(matrix_1d.calculate_index(&[2]), Some(2));
    assert_eq!(matrix_1d.calculate_index(&[3]), None); // Out of bounds

    let matrix_2d = Matrix::<u8>::new(vec![2, 3]);
    assert_eq!(matrix_2d.calculate_index(&[1, 2]), Some(5));
    assert_eq!(matrix_2d.calculate_index(&[2, 2]), None); // Out of bounds

    let matrix_3d = Matrix::<u8>::new(vec![2, 2, 2]);
    assert_eq!(matrix_3d.calculate_index(&[1, 1, 1]), Some(7));
    assert_eq!(matrix_3d.calculate_index(&[2, 2, 2]), None); // Out of bounds
}

#[test]
fn test_set_get_matrixes() {
    let mut matrix_1d = Matrix::<u8>::new(vec![3]);
    assert!(matrix_1d.set(&[1], 5).is_ok());
    assert_eq!(matrix_1d.get(&[1]), Some(&5));
    assert!(matrix_1d.set(&[3], 6).is_err()); // Out of bounds

    let mut matrix_2d = Matrix::<u8>::new(vec![2, 3]);
    assert!(matrix_2d.set(&[0, 2], 7).is_ok());
    assert_eq!(matrix_2d.get(&[0, 2]), Some(&7));
    assert!(matrix_2d.set(&[2, 2], 8).is_err()); // Out of bounds

    let mut matrix_3d = Matrix::<u8>::new(vec![2, 2, 2]);
    assert!(matrix_3d.set(&[1, 1, 1], 9).is_ok());
    assert_eq!(matrix_3d.get(&[1, 1, 1]), Some(&9));
    assert!(matrix_3d.set(&[2, 2, 2], 10).is_err()); // Out of bounds
}

#[test]
fn test_convolution() {
    assert_eq!(convolution(&[1, 2, 3], &[4, 5, 6]), [4, 13, 28, 27, 18]);
    assert_eq!(convolution(&[1, 0, -1], &[2, 5]), [2, 5, -2, -5]);
    assert_eq!(
        convolution(&[0.0, 1.0, 0.5], &[1.0, -1.0]),
        [0.0, 1.0, -0.5, -0.5]
    );
}

#[test]
fn test_average() {
    assert_eq!(average(&[1, 2, 3]), 2);
    assert_eq!(average(&[1.0, 1.0, 1.0]), 1.0);
    assert_eq!(average(&[0, 20]), 10);
    assert_eq!(average(&[-10.0, 10.0]), 0.0);
}

#[test]
fn test_sum() {
    // Should work
    let a = Matrix {
        data: vec![1, 2, 3, 4, 5, 6],
        shape: vec![2, 3],
    };
    let b = Matrix {
        data: vec![7, 8, 9, 10, 11, 12],
        shape: vec![2, 3],
    };
    let result = sum(&a, &b).unwrap();
    assert_eq!(result.data, vec![8, 10, 12, 14, 16, 18]);
    assert_eq!(result.shape, vec![2, 3]);

    // Should fail due to dimension mismatch
    let c = Matrix {
        data: vec![1, 2, 3, 4],
        shape: vec![2, 2],
    };
    let result = sum(&a, &c);
    assert_eq!(result, Err("Dimension mismatch for matrix addition"));
}

#[test]
fn test_mul() {
    // Should work
    let a = Matrix {
        data: vec![1, 2, 3, 4, 5, 6],
        shape: vec![2, 3],
    };
    let b = Matrix {
        data: vec![7, 8, 9, 10, 11, 12],
        shape: vec![3, 2],
    };
    let result = mul(&a, &b).unwrap();
    assert_eq!(result.data, vec![58, 64, 139, 154]);
    assert_eq!(result.shape, vec![2, 2]);

    // Should fail due to dimension mismatch
    let c = Matrix {
        data: vec![1, 2, 3, 4],
        shape: vec![2, 2],
    };
    let result = mul(&a, &c);
    assert_eq!(result, Err("Dimension mismatch for matrix multiplication"));
}
