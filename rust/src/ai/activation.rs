pub fn relu(x: f64) -> f64 {
    if x > 0.0 {x}
    else {0.0}
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
}
