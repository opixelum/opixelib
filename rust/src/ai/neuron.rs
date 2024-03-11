use crate::math::vector::dot;

#[derive(Debug, PartialEq)]
pub struct Neuron {
    pub weights: Option<Vec<f64>>,
    pub bias: f64,
}

impl Neuron {
    pub fn forward(&self, inputs: Vec<f64>) -> f64 {
        dot(
            inputs,
            self.weights.clone().expect("Weights are not initialized."),
        )
        .unwrap()
            + self.bias
    }
}
