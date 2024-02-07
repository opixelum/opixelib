use opixelib::math::vectors::*;

#[test]
fn test_sum() {
    assert_eq!(sum(&[1, 2, 3], &[4, 5, 6]), Ok(vec![5, 7, 9]));
    assert_eq!(
        sum(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]),
        Ok(vec![5.0, 7.0, 9.0])
    );
    assert_eq!(
        sum(&[1, 2, 3], &[4, 5]),
        Err("Vectors must be of the same size")
    );
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3], &[4, 5, 6]), Ok(32));
    assert_eq!(dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), Ok(32.0));
    assert_eq!(
        dot(&[1, 2, 3], &[4, 5]),
        Err("Vectors must be of the same size")
    );
}
