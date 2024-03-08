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

    // fn forward(&self, inputs: Vec<f64>) -> Vec<f64> {
    //    self.neurons
    //        .iter()
    //        .enumerate()
    //        .map(|(i, neuron)| {
    //            if neuron.weights.is_none() {
    //                self.neurons[i].weights = Some(vec![0.5; inputs.len()]);
    //            }

    //            neuron.forward(inputs.clone())
    //        })
    //        .collect()
    //}
}
