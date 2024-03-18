pub type Loss = fn(target: Vec<f64>, prediction: Vec<f64>) -> f64;

pub fn mean_squared_error(target: Vec<f64>, prediction: Vec<f64>) -> f64 {
    if prediction.len() != target.len() {
        panic!("Prediction and target vectors must be of the same length.")
    }

    let mut sum = 0.0;
    for i in 0..target.len() {
        sum += (target[i] - prediction[i]).powf(2.0)
    }
    sum / target.len() as f64
}
