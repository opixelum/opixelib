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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut int_array = [7, 3, 9, 1, 5, 4, 2, 8, 6];
        insertion_sort(&mut int_array);
        assert_eq!(int_array, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut float_array = [3.14, 2.71, 1.41, 1.73];
        insertion_sort(&mut float_array);
        assert_eq!(float_array, [1.41, 1.73, 2.71, 3.14]);

        let mut char_array = ['d', 'a', 'c', 'b'];
        insertion_sort(&mut char_array);
        assert_eq!(char_array, ['a', 'b', 'c', 'd']);

        let mut string_array = ["john", "alex", "chris", "ben"];
        insertion_sort(&mut string_array);
        assert_eq!(string_array, ["alex", "ben", "chris", "john"]);

        let mut string_array = [
            String::from("doe"),
            String::from("johnson"),
            String::from("o'neil"),
            String::from("hilton")
        ];
        insertion_sort(&mut string_array);
        assert_eq!(string_array, [
            String::from("doe"),
            String::from("hilton"),
            String::from("johnson"),
            String::from("o'neil")
        ]);
    }
}
