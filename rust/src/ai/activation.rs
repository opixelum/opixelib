use std::f32::consts::E;

pub type Activation = fn(f64) -> f64;

pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

pub fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.01 * x
    }
}

pub fn softmax(values: &[f64]) -> Vec<f64> {
    let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    let exps: Vec<f64> = values.iter().map(|&x| (x - max_val).exp()).collect();
    let sum_exps: f64 = exps.iter().sum();

    exps.iter().map(|&exp_val| exp_val / sum_exps).collect()
}

pub fn binary_step(x: f64) -> f64 {
    if x >= 0.0 {
        1.0
    } else {
        0.0
    }
}

pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + E.powf(-x as f32)) as f64
}
