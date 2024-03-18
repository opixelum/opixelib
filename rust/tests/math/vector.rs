use opixelib::math::vector::*;

#[test]
fn test_sum() {
    assert_eq!(sum(&[1, 2, 3], &[4, 5, 6]), Ok(vec![5, 7, 9]));
    assert_eq!(
        sum(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]),
        Ok(vec![5.0, 7.0, 9.0])
    );
    assert_eq!(
        sum(&[1, 2, 3], &[4, 5]),
        Err("Vectors must be of the same length.")
    );
}

#[test]
fn test_dot() {
    assert_eq!(dot(vec![1, 2, 3], vec![4, 5, 6]), Ok(32));
    assert_eq!(dot(vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]), Ok(32.0));
    assert_eq!(
        dot(vec![1, 2, 3], vec![4, 5]),
        Err("Vectors must be of the same length.")
    );
}

#[test]
fn test_sub() {
    assert_eq!(sub(vec![1, 2, 3], vec![1, 2, 3]), Ok(vec![0, 0, 0]));
    assert_eq!(sub(vec![1, 2, 3], vec![1, 1, 1]), Ok(vec![0, 1, 2]));
    assert_eq!(
        sub(vec![1.0, 2.0, 3.0], vec![1.0, 2.0, 3.0]),
        Ok(vec![0.0, 0.0, 0.0])
    );
    assert_eq!(
        sub(vec![1, 2, 3], vec![4, 5]),
        Err("Vectors must be of the same length.")
    );
}
