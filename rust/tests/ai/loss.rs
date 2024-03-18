use opixelib::ai::loss::*;

#[test]
fn test_mean_squared_error() {
    assert_eq!(
        mean_squared_error(
            vec![60.0, 70.0, 80.0, 85.0, 90.0],
            vec![58.0, 71.0, 79.0, 87.0, 92.0]
        ),
        2.8
    )
}

#[test]
#[should_panic]
fn test_mean_squared_error_different_vector_size() {
    assert_eq!(mean_squared_error(vec![0.0, 0.0], vec![0.0]), 0.0)
}
