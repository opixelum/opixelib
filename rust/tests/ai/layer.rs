use opixelib::ai::activation::*;
use opixelib::ai::layer::*;
use opixelib::ai::neuron::Neuron;

#[test]
fn test_new() {
    let perceptron = Layer::new(1, binary_step);
    assert_eq!(
        perceptron,
        Layer {
            neurons: vec![Neuron {
                weights: None,
                bias: 0.0
            }],
            activation: binary_step
        }
    )
}
