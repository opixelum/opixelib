use opixelib::ai::activation::*;
use opixelib::ai::layer::*;
use opixelib::ai::neuron::Neuron;

#[test]
fn test_layer_forward() {
    let neuron: Neuron = Neuron {
        inputs: vec![1.0, 2.0, 3.0],
        weights: vec![4.0, 5.0, 6.0],
        bias: 10.0,
    };

    let perceptron = Layer {
        neurons: vec![neuron],
        activation: binary_step,
    };

    assert_eq!(perceptron.forward(), vec![1.0])
}
