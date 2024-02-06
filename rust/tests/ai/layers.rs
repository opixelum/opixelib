use opixelib::ai::layers::*;
use opixelib::ai::tensors::Tensor;

#[test]
fn test_flatten_forward() {
    let mut input = Tensor::<u8>::new(vec![2, 2, 2]);
    input
        .set(&[0, 0, 0], 55)
        .expect("Failed to set flatten input");
    input
        .set(&[0, 1, 0], 55)
        .expect("Failed to set flatten input");
    input
        .set(&[1, 1, 1], 55)
        .expect("Failed to set flatten input");

    let output = Flatten.forward(&input);
    assert_eq!(output.data, vec![55, 0, 55, 0, 0, 0, 0, 55]);
    assert_eq!(output.shape, vec![8]);
}
