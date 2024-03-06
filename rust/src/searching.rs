use std::fmt::Display;

pub fn binary_search<T: PartialOrd + Clone + Display>(
    array: &[T],
    target: T,
) -> Result<usize, String> {
    if array.is_empty() {
        return Err("Empty array".to_string());
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let middle = (left + right) / 2;

        if array[middle] < target {
            left = middle + 1
        } else if array[middle] > target {
            right = middle - 1
        } else {
            return Ok(middle);
        }
    }

    Err(format!("{} not found in array", target))
}
