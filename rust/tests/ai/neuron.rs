use opixelib::ai::neuron::*;

#[test]
fn test_forward() {
    let neuron = Neuron {
        weights: Some(vec![1.0, 2.0]),
        bias: 1.0,
    };
    assert_eq!(neuron.forward(vec![1.0, 2.0]), 6.0)
}

#[test]
#[should_panic]
fn test_forward_different_vectors_sizes() {
    let neuron = Neuron {
        weights: Some(vec![1.0]),
        bias: 1.0,
    };
    assert_eq!(neuron.forward(vec![1.0, 2.0]), 6.0)
}
