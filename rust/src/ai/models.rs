use crate::ai::{data::Dataset, layers::Layer, tensors::Tensor};

pub trait Model<T> {
    fn train(&mut self, dataset: &dyn Dataset<T>, epochs: usize, learning_rate: f64);
    fn predict(&self, input: &Tensor<T>) -> Tensor<T>;
}

pub struct Sequential<T> {
    layers: Vec<Box<dyn Layer<T>>>,
}

impl<T> Sequential<T> {
    fn new(layers: Vec<Box<dyn Layer<T>>>) -> Self {
        Sequential { layers }
    }

    // fn train(&mut self, dataset: &Dataset<T>, epochs: usize, learning_rate: f64) {
    //     // Iterate over the dataset
    //     // Forward pass
    //     // Backpropagation
    // }

    // fn predict(&self, input: &Tensor<T>) -> Tensor<T> {
    //     // Pass the input through the model and return the output
    // }
}
