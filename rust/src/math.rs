use num_traits::cast::NumCast;
use std::ops::{Add, AddAssign, Div, Mul};

pub fn pairwise_operation<T, F>(a: &[T], b: &[T], mut operator: F) -> Vec<Vec<T>>
where
    T: Clone + Default,
    F: FnMut(&T, &T) -> T,
{
    let mut result = vec![vec![T::default(); b.len()]; a.len()];
    for (i, a_elem) in a.iter().enumerate() {
        for (j, b_elem) in b.iter().enumerate() {
            result[i][j] = operator(a_elem, b_elem);
        }
    }
    result
}

pub fn diagonal_operation<T, F>(matrix: &[Vec<T>], mut operator: F) -> Vec<T>
where
    T: Clone + Default,
    F: FnMut(T, T) -> T,
{
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };
    let mut result = vec![T::default(); rows + cols - 1];

    for i in 0..rows {
        for j in 0..cols {
            let result_index = i + j;
            if result_index < result.len() {
                result[result_index] = operator(result[result_index].clone(), matrix[i][j].clone());
            }
        }
    }

    result
}
pub fn convolution<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Clone + Default,
{
    let mut outer_product = pairwise_operation(&f, &g, |a, b| a.clone() * b.clone());
    diagonal_operation(&mut outer_product, |a, b| a + b)
}

pub fn average<T>(values: &[T]) -> T
where
    T: Mul<Output = T> + Add<Output = T> + AddAssign + Clone + Default + Div<Output = T> + NumCast,
{
    let mut sum: T = T::default();
    for value in values {
        sum += value.clone();
    }
    sum / NumCast::from(values.len()).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convolution() {
        assert_eq!(convolution(&[1, 2, 3], &[4, 5, 6]), [4, 13, 28, 27, 18]);
        assert_eq!(convolution(&[1, 0, -1], &[2, 5]), [2, 5, -2, -5]);
        assert_eq!(
            convolution(&[0.0, 1.0, 0.5], &[1.0, -1.0]),
            [0.0, 1.0, -0.5, -0.5]
        );
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[1, 2, 3]), 2);
        assert_eq!(average(&[1.0, 1.0, 1.0]), 1.0);
        assert_eq!(average(&[0, 20]), 10);
        assert_eq!(average(&[-10.0, 10.0]), 0.0);
    }
}
