use opixelib::array::*;

// Define test data for different types
static LEFT_INT_ARRAY: [i32; 5] = [0, 1, 2, 3, 4];
static RIGHT_INT_ARRAY: [i32; 5] = [5, 6, 7, 8, 9];
static MERGED_INT_ARRAY: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

static LEFT_FLOAT_ARRAY: [f64; 3] = [1.1, 2.2, 3.3];
static RIGHT_FLOAT_ARRAY: [f64; 3] = [4.4, 5.5, 6.6];
static MERGED_FLOAT_ARRAY: [f64; 6] = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6];

static LEFT_CHAR_ARRAY: [char; 2] = ['a', 'b'];
static RIGHT_CHAR_ARRAY: [char; 2] = ['c', 'd'];
static MERGED_CHAR_ARRAY: [char; 4] = ['a', 'b', 'c', 'd'];

static LEFT_STRING_SLICE_ARRAY: [&str; 2] = ["alpha", "beta"];
static RIGHT_STRING_SLICE_ARRAY: [&str; 2] = ["gamma", "delta"];
static MERGED_STRING_SLICE_ARRAY: [&str; 4] = ["alpha", "beta", "gamma", "delta"];

static LEFT_STRING_ARRAY: [&str; 2] = ["doe", "hilton"];
static RIGHT_STRING_ARRAY: [&str; 2] = ["johnson", "o'neil"];
static MERGED_STRING_ARRAY: [&str; 4] = ["doe", "hilton", "johnson", "o'neil"];

#[test]
fn test_merge_arrays() {
    let mut dest = [0; 10];
    merge_arrays(&LEFT_INT_ARRAY, &RIGHT_INT_ARRAY, &mut dest);
    assert_eq!(dest, MERGED_INT_ARRAY);

    let mut dest = [0.0; 6];
    merge_arrays(&LEFT_FLOAT_ARRAY, &RIGHT_FLOAT_ARRAY, &mut dest);
    assert_eq!(dest, MERGED_FLOAT_ARRAY);

    let mut dest = [' '; 4];
    merge_arrays(&LEFT_CHAR_ARRAY, &RIGHT_CHAR_ARRAY, &mut dest);
    assert_eq!(dest, MERGED_CHAR_ARRAY);

    let mut dest = [""; 4];
    merge_arrays(
        &LEFT_STRING_SLICE_ARRAY,
        &RIGHT_STRING_SLICE_ARRAY,
        &mut dest,
    );
    assert_eq!(dest, MERGED_STRING_SLICE_ARRAY);

    let left_string_array: Vec<String> = LEFT_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    let right_string_array: Vec<String> = RIGHT_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    let mut dest: Vec<String> = vec![String::new(); 4];
    merge_arrays(&left_string_array, &right_string_array, &mut dest);
    let merged_string_array: Vec<String> = MERGED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    assert_eq!(dest, merged_string_array);
}

#[test]
fn test_copy_arrays() {
    let mut to = [0; 10];
    copy_arrays(&MERGED_INT_ARRAY, &mut to);
    assert_eq!(MERGED_INT_ARRAY, to);

    let mut to = [0.0; 6];
    copy_arrays(&MERGED_FLOAT_ARRAY, &mut to);
    assert_eq!(MERGED_FLOAT_ARRAY, to);

    let mut to = [' '; 4];
    copy_arrays(&MERGED_CHAR_ARRAY, &mut to);
    assert_eq!(MERGED_CHAR_ARRAY, to);

    let mut to = [""; 4];
    copy_arrays(&MERGED_STRING_SLICE_ARRAY, &mut to);
    assert_eq!(MERGED_STRING_SLICE_ARRAY, to);

    let mut to: Vec<String> = vec![String::new(); 4];
    let merged_string_array: Vec<String> = MERGED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    copy_arrays(&merged_string_array, &mut to);
    assert_eq!(merged_string_array, to);
}

#[test]
fn test_invert_array() {
    let mut inverted_int = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
    invert_array(&mut inverted_int);
    assert_eq!(inverted_int, MERGED_INT_ARRAY);

    let mut inverted_float = [6.6, 5.5, 4.4, 3.3, 2.2, 1.1];
    invert_array(&mut inverted_float);
    assert_eq!(inverted_float, MERGED_FLOAT_ARRAY);

    let mut inverted_char = ['d', 'c', 'b', 'a'];
    invert_array(&mut inverted_char);
    assert_eq!(inverted_char, MERGED_CHAR_ARRAY);

    let mut inverted_string_slice = ["delta", "gamma", "beta", "alpha"];
    invert_array(&mut inverted_string_slice);
    assert_eq!(inverted_string_slice, MERGED_STRING_SLICE_ARRAY);

    let mut inverted_string = vec![
        String::from("o'neil"),
        String::from("johnson"),
        String::from("hilton"),
        String::from("doe"),
    ];
    let merged_string: Vec<String> = MERGED_STRING_ARRAY.iter().map(|&s| s.into()).collect();
    invert_array(&mut inverted_string);
    assert_eq!(inverted_string, merged_string);
}
