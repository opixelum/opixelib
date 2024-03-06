use crate::math::vector::dot;

pub struct Neuron {
    pub inputs: Vec<f64>,
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Neuron {
    pub fn forward(&self) -> f64 {
        dot(&self.inputs, &self.weights).unwrap() + self.bias
    }
}
