use crate::array::{copy_arrays, merge_arrays};

pub fn insertion_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let size = array.len();

    for i in 1..size {
        let current_element = array[i].clone();
        let mut new_position = i as isize - 1;

        while new_position >= 0 && array[new_position as usize] > current_element {
            array[(new_position + 1) as usize] = array[new_position as usize].clone();
            new_position -= 1;
        }

        array[(new_position + 1) as usize] = current_element;
    }
}

pub fn odd_even_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let size = array.len();
    let mut sorted = false;

    while !sorted {
        sorted = true;

        // Even-odd phase
        for i in (0..size - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }

        // Odd-even phase
        for i in (1..size - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

pub fn comb_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let mut sorted = false;
    let mut gap: f64 = array.len() as f64;

    while gap > 1.0 || !sorted {
        sorted = true;
        gap /= 1.3;
        if gap < 1.0 {
            gap = 1.0;
        }

        for i in 0..array.len() - gap as usize {
            if array[i] > array[i + gap as usize] {
                array.swap(i, i + gap as usize);
                sorted = false
            }
        }
    }
}

pub fn cocktail_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let mut sorted = false;

    while !sorted {
        sorted = true;

        for i in 0..array.len() - 1 {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }

        for i in (array.len() - 1..=0).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                sorted = false;
            }
        }
    }
}

pub fn merge_sort<T: PartialOrd + Clone>(array: &mut [T]) {
    let len = array.len();
    if len > 1 {
        let middle = array.len() / 2;
        let (left, right) = array.split_at_mut(middle);

        merge_sort(left);
        merge_sort(right);

        let mut temp = vec![left[0].clone(); len];
        merge_arrays(left, right, &mut temp);

        copy_arrays(&temp, array);
    }
}
