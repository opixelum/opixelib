use opixelib::sorting::*;

// Define your unsorted test data as static arrays or slices
static INT_ARRAY: [i32; 9] = [7, 3, 9, 1, 5, 4, 2, 8, 6];
static FLOAT_ARRAY: [f64; 4] = [3.14, 2.71, 1.41, 1.73];
static CHAR_ARRAY: [char; 4] = ['d', 'a', 'c', 'b'];
static STRING_SLICE_ARRAY: [&str; 4] = ["john", "alex", "chris", "ben"];
static STRING_ARRAY: [&str; 4] = ["o'neil", "johnson", "hilton", "doe"];

// Define your sorted test data
static SORTED_INT_ARRAY: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
static SORTED_FLOAT_ARRAY: [f64; 4] = [1.41, 1.73, 2.71, 3.14];
static SORTED_CHAR_ARRAY: [char; 4] = ['a', 'b', 'c', 'd'];
static SORTED_STRING_SLICE_ARRAY: [&str; 4] = ["alex", "ben", "chris", "john"];
static SORTED_STRING_ARRAY: [&str; 4] = ["doe", "hilton", "johnson", "o'neil"];

#[test]
fn test_insertion_sort() {
    let mut int_array = INT_ARRAY;
    insertion_sort(&mut int_array);
    assert_eq!(int_array, SORTED_INT_ARRAY);

    let mut float_array = FLOAT_ARRAY;
    insertion_sort(&mut float_array);
    assert_eq!(float_array, SORTED_FLOAT_ARRAY);

    let mut char_array = CHAR_ARRAY;
    insertion_sort(&mut char_array);
    assert_eq!(char_array, SORTED_CHAR_ARRAY);

    let mut string_slice_array = STRING_SLICE_ARRAY;
    insertion_sort(&mut string_slice_array);
    assert_eq!(string_slice_array, SORTED_STRING_SLICE_ARRAY);

    let mut string_array: Vec<String> = STRING_ARRAY.iter().map(|&s| s.into()).collect();
    insertion_sort(&mut string_array);
    let sorted_string_array: Vec<String> = SORTED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(string_array, sorted_string_array);
}

#[test]
fn test_odd_even_sort() {
    let mut int_array = INT_ARRAY;
    odd_even_sort(&mut int_array);
    assert_eq!(int_array, SORTED_INT_ARRAY);

    let mut float_array = FLOAT_ARRAY;
    odd_even_sort(&mut float_array);
    assert_eq!(float_array, SORTED_FLOAT_ARRAY);

    let mut char_array = CHAR_ARRAY;
    odd_even_sort(&mut char_array);
    assert_eq!(char_array, SORTED_CHAR_ARRAY);

    let mut string_slice_array = STRING_SLICE_ARRAY;
    odd_even_sort(&mut string_slice_array);
    assert_eq!(string_slice_array, SORTED_STRING_SLICE_ARRAY);

    let mut string_array: Vec<String> = STRING_ARRAY.iter().map(|&s| s.into()).collect();
    odd_even_sort(&mut string_array);
    let sorted_string_array: Vec<String> = SORTED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(string_array, sorted_string_array);
}

#[test]
fn test_comb_sort() {
    let mut int_array = INT_ARRAY;
    comb_sort(&mut int_array);
    assert_eq!(int_array, SORTED_INT_ARRAY);

    let mut float_array = FLOAT_ARRAY;
    comb_sort(&mut float_array);
    assert_eq!(float_array, SORTED_FLOAT_ARRAY);

    let mut char_array = CHAR_ARRAY;
    comb_sort(&mut char_array);
    assert_eq!(char_array, SORTED_CHAR_ARRAY);

    let mut string_slice_array = STRING_SLICE_ARRAY;
    comb_sort(&mut string_slice_array);
    assert_eq!(string_slice_array, SORTED_STRING_SLICE_ARRAY);

    let mut string_array: Vec<String> = STRING_ARRAY.iter().map(|&s| s.into()).collect();
    comb_sort(&mut string_array);
    let sorted_string_array: Vec<String> = SORTED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(string_array, sorted_string_array);
}

#[test]
fn test_cocktail_sort() {
    let mut int_array = INT_ARRAY;
    cocktail_sort(&mut int_array);
    assert_eq!(int_array, SORTED_INT_ARRAY);

    let mut float_array = FLOAT_ARRAY;
    cocktail_sort(&mut float_array);
    assert_eq!(float_array, SORTED_FLOAT_ARRAY);

    let mut char_array = CHAR_ARRAY;
    cocktail_sort(&mut char_array);
    assert_eq!(char_array, SORTED_CHAR_ARRAY);

    let mut string_slice_array = STRING_SLICE_ARRAY;
    cocktail_sort(&mut string_slice_array);
    assert_eq!(string_slice_array, SORTED_STRING_SLICE_ARRAY);

    let mut string_array: Vec<String> = STRING_ARRAY.iter().map(|&s| s.into()).collect();
    cocktail_sort(&mut string_array);
    let sorted_string_array: Vec<String> = SORTED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(string_array, sorted_string_array);
}

#[test]
fn test_merge_sort() {
    let mut int_array = INT_ARRAY;
    merge_sort(&mut int_array);
    assert_eq!(int_array, SORTED_INT_ARRAY);

    let mut float_array = FLOAT_ARRAY;
    merge_sort(&mut float_array);
    assert_eq!(float_array, SORTED_FLOAT_ARRAY);

    let mut char_array = CHAR_ARRAY;
    merge_sort(&mut char_array);
    assert_eq!(char_array, SORTED_CHAR_ARRAY);

    let mut string_slice_array = STRING_SLICE_ARRAY;
    merge_sort(&mut string_slice_array);
    assert_eq!(string_slice_array, SORTED_STRING_SLICE_ARRAY);

    let mut string_array: Vec<String> = STRING_ARRAY.iter().map(|&s| s.into()).collect();
    merge_sort(&mut string_array);
    let sorted_string_array: Vec<String> = SORTED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(string_array, sorted_string_array);
}
