use crate::ai::activation::*;
use crate::ai::neuron::Neuron;

pub struct Layer {
    pub neurons: Vec<Neuron>,
    pub activation: Activation,
}

impl Layer {
    pub fn forward(&self) -> Vec<f64> {
        let mut outputs = Vec::new();

        for neuron in self.neurons.iter() {
            outputs.push((self.activation)(neuron.forward()))
        }

        outputs
    }
}
