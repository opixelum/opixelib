use opixelib::ai::activation::*;
use opixelib::ai::layer::*;
use opixelib::ai::neuron::Neuron;

#[test]
fn test_new() {
    assert_eq!(
        Layer::new(1, binary_step),
        Layer {
            neurons: vec![Neuron {
                weights: None,
                bias: 0.0
            }],
            activation: binary_step
        }
    );
}

#[test]
fn test_forward() {
    let mut perceptron = Layer::new(1, binary_step);
    assert_eq!(perceptron.forward(vec![1.0, 2.0]), vec![1.0]);
    assert_eq!(perceptron.forward(vec![-1.0, -2.0]), vec![0.0])
}
