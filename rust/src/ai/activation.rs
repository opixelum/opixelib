pub fn relu(x: f64) -> f64 {
    if x > 0.0 {x}
    else {0.0}
}

pub fn leaky_relu(x: f64) -> f64 {
    if x > 0.0 {x}
    else {0.01 * x}
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
}
