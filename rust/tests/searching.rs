use lazy_static::lazy_static;
use opixelib::searching::*;

lazy_static! {
    static ref INT_ARRAY_EVEN: [i32; 6] = [1, 3, 5, 7, 9, 11];
    static ref INT_ARRAY_ODD: [i32; 7] = [2, 4, 6, 8, 10, 12, 14];
    static ref INT_ARRAY_EMPTY: [i32; 0] = [];
    static ref FLOAT_ARRAY_EVEN: [f64; 6] = [1.0, 3.0, 5.0, 7.0, 9.0, 11.0];
    static ref FLOAT_ARRAY_ODD: [f64; 7] = [2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0];
    static ref FLOAT_ARRAY_EMPTY: [f64; 0] = [];
    static ref CHAR_ARRAY_EVEN: [char; 6] = ['a', 'b', 'c', 'd', 'e', 'f'];
    static ref CHAR_ARRAY_ODD: [char; 7] = ['g', 'h', 'i', 'j', 'k', 'l', 'm'];
    static ref CHAR_ARRAY_EMPTY: [char; 0] = [];
    static ref STR_ARRAY_EVEN: [&'static str; 6] =
        ["apple", "banana", "cherry", "date", "fig", "grape"];
    static ref STR_ARRAY_ODD: [&'static str; 7] = [
        "honeydew",
        "kiwi",
        "lemon",
        "mango",
        "nectarine",
        "orange",
        "papaya"
    ];
    static ref STR_ARRAY_EMPTY: [&'static str; 0] = [];
    static ref STRING_ARRAY_EVEN: Vec<String> = vec![
        "apple".into(),
        "banana".into(),
        "cherry".into(),
        "date".into(),
        "fig".into(),
        "grape".into()
    ];
    static ref STRING_ARRAY_ODD: Vec<String> = vec![
        "honeydew".into(),
        "kiwi".into(),
        "lemon".into(),
        "mango".into(),
        "nectarine".into(),
        "orange".into(),
        "papaya".into()
    ];
    static ref STRING_ARRAY_EMPTY: Vec<String> = vec![];
}

#[test]
fn test_binary_search() {
    // Test with integers
    assert_eq!(binary_search(&*INT_ARRAY_EVEN, 1), Ok(0));
    assert_eq!(
        binary_search(&*INT_ARRAY_EVEN, 4),
        Err("4 not found in array".to_string())
    );
    assert_eq!(binary_search(&*INT_ARRAY_ODD, 2), Ok(0));
    assert_eq!(
        binary_search(&*INT_ARRAY_ODD, 11),
        Err("11 not found in array".to_string())
    );
    assert_eq!(
        binary_search(&*INT_ARRAY_EMPTY, 1),
        Err("Empty array".to_string())
    );

    // Test with floats
    assert_eq!(binary_search(&*FLOAT_ARRAY_EVEN, 3.0), Ok(1));
    assert_eq!(
        binary_search(&*FLOAT_ARRAY_EVEN, 4.0),
        Err("4 not found in array".to_string())
    );
    assert_eq!(binary_search(&*FLOAT_ARRAY_ODD, 4.0), Ok(1));
    assert_eq!(
        binary_search(&*FLOAT_ARRAY_ODD, 11.0),
        Err("11 not found in array".to_string())
    );
    assert_eq!(
        binary_search(&*FLOAT_ARRAY_EMPTY, 1.0),
        Err("Empty array".to_string())
    );

    // Test with char
    assert_eq!(binary_search(&*CHAR_ARRAY_EVEN, 'c'), Ok(2));
    assert_eq!(
        binary_search(&*CHAR_ARRAY_EVEN, 'z'),
        Err("z not found in array".to_string())
    );
    assert_eq!(binary_search(&*CHAR_ARRAY_ODD, 'i'), Ok(2));
    assert_eq!(
        binary_search(&*CHAR_ARRAY_ODD, 'z'),
        Err("z not found in array".to_string())
    );
    assert_eq!(
        binary_search(&*CHAR_ARRAY_EMPTY, 'a'),
        Err("Empty array".to_string())
    );

    // Test with &str
    assert_eq!(binary_search(&*STR_ARRAY_EVEN, "date"), Ok(3));
    assert_eq!(
        binary_search(&*STR_ARRAY_EVEN, "quince"),
        Err("quince not found in array".to_string())
    );
    assert_eq!(binary_search(&*STR_ARRAY_ODD, "mango"), Ok(3));
    assert_eq!(
        binary_search(&*STR_ARRAY_ODD, "raspberry"),
        Err("raspberry not found in array".to_string())
    );
    assert_eq!(
        binary_search(&*STR_ARRAY_EMPTY, "a"),
        Err("Empty array".to_string())
    );

    // Test with String
    assert_eq!(binary_search(&STRING_ARRAY_EVEN, "fig".to_string()), Ok(4));
    assert_eq!(
        binary_search(&STRING_ARRAY_EVEN, "quince".to_string()),
        Err("quince not found in array".to_string())
    );
    assert_eq!(
        binary_search(&STRING_ARRAY_ODD, "nectarine".to_string()),
        Ok(4)
    );
    assert_eq!(
        binary_search(&STRING_ARRAY_ODD, "raspberry".to_string()),
        Err("raspberry not found in array".to_string())
    );
    assert_eq!(
        binary_search(&STRING_ARRAY_EMPTY, "a".to_string()),
        Err("Empty array".to_string())
    );
}
