use crate::ai::tensors::Tensor;
use image::{io::Reader as ImageReader, GenericImageView, Pixel};

pub fn image_to_tensor(path: &str) -> Tensor<f64> {
    // Load the image from the given path
    let img = ImageReader::open(path)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let (width, height) = img.dimensions();
    let color_channels = 4; // For RGBA data

    // Prepare the data vector and fill it with pixel values
    let mut data = Vec::with_capacity((width * height * color_channels) as usize);
    for (_, _, pixel) in img.pixels() {
        // Get rgba values
        let rgba = pixel.to_rgba();

        // Normalize pixel values and push to data vector
        data.push(rgba[0] as f64);
        data.push(rgba[1] as f64);
        data.push(rgba[2] as f64);
        data.push(rgba[3] as f64);
    }

    Tensor {
        data,
        shape: vec![height as usize, width as usize, color_channels as usize],
    }
}

pub fn label_to_tensor(label: &str) -> Tensor<f64> {
    let mut data = Vec::with_capacity(label.len());

    for character in label.chars() {
        data.push(character as u32 as f64);
    }

    Tensor {
        data,
        shape: vec![label.len()],
    }
}

pub trait Dataset<T> {
    fn next_batch(&mut self, batch_size: usize) -> Vec<T>;
    // Placeholder for other common methods like shuffling, splitting, etc.
}

// pub struct ImageDataset<T> {
//     features: Vec<Tensor<T>>,
//     labels: Vec<Tensor<T>>,
//     current_index: usize, // To keep track of the current position for batching
// }
//
// impl<T> ImageDataset<T> {
//     pub fn new(path: &str) -> Self {
//         // Read data and labels from the folder, converting them to tensors
//         // For this example, we're using dummy data
//         let features = vec![/* tensors representing your features */];
//         let labels = vec![/* tensors representing your labels */];
//
//         ImageDataset {
//             features,
//             labels,
//             current_index: 0,
//         }
//     }
// }
//
// impl<T> Dataset<(Tensor<T>, Tensor<T>)> for ImageDataset<T> {
//     fn next_batch(&mut self, batch_size: usize) -> Vec<(Tensor<T>, Tensor<T>)> {
//         let mut batch = Vec::new();
//
//         for _ in 0..batch_size {
//             if self.current_index >= self.features.len() {
//                 break; // Stop if we reach the end of the dataset
//             }
//
//             let feature = self.features[self.current_index].clone();
//             let label = self.labels[self.current_index].clone();
//             batch.push((feature, label));
//
//             self.current_index += 1;
//         }
//
//         // Reset current_index if end of dataset is reached
//         if self.current_index >= self.features.len() {
//             self.current_index = 0;
//         }
//
//         batch
//     }
//
//     // Placeholder for other methods like shuffle, split, etc.
// }
