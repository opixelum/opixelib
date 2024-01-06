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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu() {
        assert_eq!(relu(-1.0), 0.0);
        assert_eq!(relu(0.0), 0.0);
        assert_eq!(relu(1.0), 1.0);
    }

    #[test]
    fn test_leaky_relu() {
        assert_eq!(leaky_relu(-1.0), -0.01);
        assert_eq!(leaky_relu(0.0), 0.0);
        assert_eq!(leaky_relu(1.0), 1.0);
    }

    #[test]
    fn test_softmax() {
        assert_eq!(
            softmax(vec![-3.5, -2.37, 1.54, 5.23].as_ref()),
            vec![
                0.00015762195671368656,
                0.0004879434348630996,
                0.02434786602197348,
                0.9750065685864497
            ]
        );
        assert_eq!(
            softmax(vec![1.3, 5.1, 2.2, 0.7, 1.1].as_ref()),
            vec![
                0.020190464732580682,
                0.9025376890165725,
                0.049660529871960124,
                0.011080761983386346,
                0.01653055439550022
            ]
        );
        assert_eq!(
            softmax(vec![-0.1, 3.8, 1.1, -0.3].as_ref()),
            vec![
                0.018334730910579303,
                0.9057806106734848,
                0.06087345037003523,
                0.01501120804590075
            ]
        );
    }
}
