use opixelib::ai::data::*;

#[test]
fn test_image_to_tensor() {
    // 2x2 white square
    let tensor = image_to_tensor("data/images/2x2-white-square.png");
    assert_eq!(tensor.shape, vec![2, 2, 4]);
    for i in 0..2 {
        for j in 0..2 {
            let r = tensor.get(&[i, j, 0]).unwrap();
            let g = tensor.get(&[i, j, 1]).unwrap();
            let b = tensor.get(&[i, j, 2]).unwrap();
            let a = tensor.get(&[i, j, 3]).unwrap();

            // All channels should be high
            assert_eq!(r, &255.0);
            assert_eq!(g, &255.0);
            assert_eq!(b, &255.0);
            assert_eq!(a, &255.0);
        }
    }

    // 3x3 red square
    let tensor = image_to_tensor("data/images/3x3-red-square.png");
    assert_eq!(tensor.shape, vec![3, 3, 4]);
    for i in 0..3 {
        for j in 0..3 {
            let r = tensor.get(&[i, j, 0]).unwrap();
            let g = tensor.get(&[i, j, 1]).unwrap();
            let b = tensor.get(&[i, j, 2]).unwrap();
            let a = tensor.get(&[i, j, 3]).unwrap();

            // Red and alpha channel should be high, green and blue channels should be zero
            assert_eq!(r, &255.0);
            assert_eq!(g, &0.0);
            assert_eq!(b, &0.0);
            assert_eq!(a, &255.0);
        }
    }

    // 5x5 alternating red, green, and blue square
    let tensor = image_to_tensor("data/images/5x5-rgb-square.png");
    assert_eq!(tensor.shape, vec![5, 5, 4]);
    for i in 0..5 {
        for j in 0..5 {
            let r = tensor.get(&[i, j, 0]).unwrap();
            let g = tensor.get(&[i, j, 1]).unwrap();
            let b = tensor.get(&[i, j, 2]).unwrap();
            let a = tensor.get(&[i, j, 3]).unwrap();

            // Calculate the expected color based on row and column
            let expected_color = match (i, j % 3) {
                (0, 0) | (1, 1) | (2, 2) | (3, 0) | (4, 1) => ("red", &255.0, &0.0, &0.0),
                (0, 1) | (1, 2) | (2, 0) | (3, 1) | (4, 2) => ("green", &0.0, &255.0, &0.0),
                (0, 2) | (1, 0) | (2, 1) | (3, 2) | (4, 0) => ("blue", &0.0, &0.0, &255.0),
                _ => panic!("Invalid color pattern"),
            };

            match expected_color {
                ("red", er, eg, eb) => {
                    assert_eq!(r, er);
                    assert_eq!(g, eg);
                    assert_eq!(b, eb);
                }
                ("green", er, eg, eb) => {
                    assert_eq!(r, er);
                    assert_eq!(g, eg);
                    assert_eq!(b, eb);
                }
                ("blue", er, eg, eb) => {
                    assert_eq!(r, er);
                    assert_eq!(g, eg);
                    assert_eq!(b, eb);
                }
                _ => (),
            }
            assert_eq!(a, &255.0);
        }
    }

    // 4x2 gradient red rectangle
    let tensor = image_to_tensor("data/images/4x2-gradient-red-rectangle.png");
    assert_eq!(tensor.shape, vec![2, 4, 4]);
    for i in 0..2 {
        for j in 0..4 {
            let r = tensor.get(&[i, j, 0]).unwrap();
            let g = tensor.get(&[i, j, 1]).unwrap();
            let b = tensor.get(&[i, j, 2]).unwrap();
            let a = tensor.get(&[i, j, 3]).unwrap();

            // Calculate the expected color based on row and column
            let expected_color = match (i, j) {
                (0, 0) | (0, 1) | (0, 2) | (0, 3) => ("light red", &255.0, &0.0, &0.0),
                (1, 0) | (1, 1) | (1, 2) | (1, 3) => ("dark red", &125.0, &0.0, &0.0),
                _ => panic!("Invalid color pattern"),
            };

            match expected_color {
                ("light red", er, eg, eb) => {
                    assert_eq!(r, er);
                    assert_eq!(g, eg);
                    assert_eq!(b, eb);
                }
                ("dark red", er, eg, eb) => {
                    assert_eq!(r, er);
                    assert_eq!(g, eg);
                    assert_eq!(b, eb);
                }
                _ => (),
            }
        }
    }
}

#[test]
fn test_label_to_tensor() {
    let tensor = label_to_tensor("0");
    assert_eq!(tensor.data, vec![48.0]);
    assert_eq!(tensor.shape, vec![1]);

    let tensor = label_to_tensor("cat");
    assert_eq!(tensor.data, vec![99.0, 97.0, 116.0]);
    assert_eq!(tensor.shape, vec![3]);

    let tensor = label_to_tensor("parastratiosphecomyia stratiosphecomyioides");
    assert_eq!(
        tensor.data,
        vec![
            112.0, 97.0, 114.0, 97.0, 115.0, 116.0, 114.0, 97.0, 116.0, 105.0, 111.0, 115.0, 112.0,
            104.0, 101.0, 99.0, 111.0, 109.0, 121.0, 105.0, 97.0, 32.0, 115.0, 116.0, 114.0, 97.0,
            116.0, 105.0, 111.0, 115.0, 112.0, 104.0, 101.0, 99.0, 111.0, 109.0, 121.0, 105.0,
            111.0, 105.0, 100.0, 101.0, 115.0
        ]
    );
    assert_eq!(tensor.shape, vec![43]);
}
