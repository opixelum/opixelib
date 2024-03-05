pub fn merge_arrays<T: PartialOrd + Clone>(left: &[T], right: &[T], dest: &mut [T]) {
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut merged_idx = 0;

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            dest[merged_idx] = left[left_idx].clone();
            left_idx += 1;
        } else {
            dest[merged_idx] = right[right_idx].clone();
            right_idx += 1;
        }
        merged_idx += 1;
    }

    while left_idx < left.len() {
        dest[merged_idx] = left[left_idx].clone();
        left_idx += 1;
        merged_idx += 1;
    }

    while right_idx < right.len() {
        dest[merged_idx] = right[right_idx].clone();
        right_idx += 1;
        merged_idx += 1;
    }
}

pub fn copy_arrays<T: Clone>(from: &[T], to: &mut [T]) {
    for (i, from) in from.iter().enumerate() {
        to[i] = from.clone()
    }
}

pub fn invert_array<T>(array: &mut [T]) {
    for i in 0..(array.len() / 2) {
        array.swap(i, array.len() - 1 - i);
    }
}
