use crate::ai::activation::Activation;
use crate::ai::neuron::Neuron;

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub activation: Activation,
}

impl Layer {
    pub fn new(number_of_neurons: u128, activation: Activation) -> Self {
        let neurons: Vec<Neuron> = (0..number_of_neurons)
            .map(|_| Neuron {
                weights: None,
                bias: 0.0,
            })
            .collect();

        Layer {
            neurons,
            activation,
        }
    }

    pub fn forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        let mut outputs = vec![];

        for neuron in self.neurons.iter_mut() {
            if neuron.weights.is_none() {
                neuron.weights = Some(vec![1.0; inputs.len()])
            }
            outputs.push((self.activation)(neuron.forward(inputs.clone())))
        }

        outputs
    }
}
