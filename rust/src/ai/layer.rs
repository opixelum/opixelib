use crate::ai::tensor::Tensor;
use std::iter::Sum;
use std::ops::{Add, Mul};

pub trait Layer<T> {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T>;
    // TODO: Add backpropagation
}

pub struct Flatten;

impl<T: Default + Clone> Layer<T> for Flatten {
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        Tensor {
            data: input.data.clone(),
            shape: vec![input.shape.iter().product()],
        }
    }
}

pub struct Dense<T> {
    weights: Tensor<T>,
    biases: Tensor<T>,
}

impl<T> Dense<T>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Sum, // other necessary traits
{
    pub fn new(input_size: usize, output_size: usize) -> Dense<T> {
        let weights = Tensor::new(vec![output_size, input_size]); // Initialize with random values
        let biases = Tensor::new(vec![output_size]); // Initialize with zeros or small constants

        Dense { weights, biases }
    }
}

/*
impl<T> Layer<T> for Dense<T>
where
    T: Default + Clone + Add<Output = T> + Mul<Output = T> + Sum, // other necessary traits
{
    fn forward(&self, input: &Tensor<T>) -> Tensor<T> {
        // Implement the forward pass for Dense layer
        // Typically involves matrix multiplication with input and adding biases
        // Return the resulting tensor
    }
}
*/