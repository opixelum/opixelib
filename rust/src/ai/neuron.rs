use crate::math::vector::dot;

pub struct Neuron {
    pub inputs: Vec<f64>,
    pub weights: Vec<f64>,
}

pub fn forward(neuron: Neuron) -> Result<f64, &'static str> {
    dot(&neuron.inputs, &neuron.weights)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward() {
        let neuron: Neuron = Neuron {
            inputs: vec![1.0, 2.0, 3.0],
            weights: vec![4.0, 5.0, 6.0],
        };
        assert_eq!(forward(neuron), Ok(32.0))
    }
}
