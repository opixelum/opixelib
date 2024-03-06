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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let neuron: Neuron = Neuron {
            inputs: vec![1.0, 2.0, 3.0],
            weights: vec![4.0, 5.0, 6.0],
            bias: 10.0,
        };
        assert_eq!(neuron.forward(), 42.0)
    }
}
