use opixelib::ai::activation::*;

#[test]
fn test_relu() {
    assert_eq!(relu(-1.0), 0.0);
    assert_eq!(relu(0.0), 0.0);
    assert_eq!(relu(1.0), 1.0);
}

#[test]
fn test_leaky_relu() {
    assert_eq!(leaky_relu(-1.0), -0.01);
    assert_eq!(leaky_relu(0.0), 0.0);
    assert_eq!(leaky_relu(1.0), 1.0);
}

#[test]
fn test_softmax() {
    assert_eq!(
        softmax(vec![-3.5, -2.37, 1.54, 5.23].as_ref()),
        vec![
            0.00015762195671368656,
            0.0004879434348630996,
            0.02434786602197348,
            0.9750065685864497
        ]
    );
    assert_eq!(
        softmax(vec![1.3, 5.1, 2.2, 0.7, 1.1].as_ref()),
        vec![
            0.020190464732580682,
            0.9025376890165725,
            0.049660529871960124,
            0.011080761983386346,
            0.01653055439550022
        ]
    );
    assert_eq!(
        softmax(vec![-0.1, 3.8, 1.1, -0.3].as_ref()),
        vec![
            0.018334730910579303,
            0.9057806106734848,
            0.06087345037003523,
            0.01501120804590075
        ]
    );
}

#[test]
fn test_binary_step() {
    assert_eq!(binary_step(-1.0), 0.0);
    assert_eq!(binary_step(0.0), 1.0);
    assert_eq!(binary_step(1.0), 1.0);
}

#[test]
fn test_sigmoid() {
    assert_eq!(sigmoid(1.0), 0.7310586055954523);
    assert_eq!(sigmoid(10.0), 0.9999545833234493);
    assert_eq!(sigmoid(-5.0), 0.006692852151141873);
}
