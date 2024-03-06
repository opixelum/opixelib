use once_cell::sync::Lazy;
use opixelib::queue::*;

static EMPTY_QUEUE: Queue<i8> = Queue {
    items: Vec::new(),
    start: 0,
    size: 0,
    capacity: 1,
};

static FULL_INT_QUEUE: Lazy<Queue<i8>> = Lazy::new(|| Queue {
    items: vec![1],
    start: 0,
    size: 1,
    capacity: 1,
});

#[test]
fn test_is_empty() {
    assert!(EMPTY_QUEUE.is_empty());
    assert!(!FULL_INT_QUEUE.is_empty());
}

#[test]
fn test_is_full() {
    assert!(!EMPTY_QUEUE.is_full());
    assert!(FULL_INT_QUEUE.is_full());
}

#[test]
fn test_enqueue() {
    let mut queue = Queue {
        items: vec![0, 0],
        start: 0,
        size: 1,
        capacity: 2,
    };
    let expected_result = Queue {
        items: vec![0, 1],
        start: 0,
        size: 2,
        capacity: 2,
    };
    assert_eq!(queue.enqueue(1), Ok(1));
    assert_eq!(queue, expected_result);
    assert_eq!(queue.enqueue(1), Err("Unable to enqueue a full queue"));
}

#[test]
fn test_dequeue() {
    let mut queue = Queue {
        items: vec![0],
        start: 0,
        size: 1,
        capacity: 1,
    };
    let expected_result = Queue {
        items: vec![0],
        start: 0,
        size: 0,
        capacity: 1,
    };
    assert_eq!(queue.dequeue(), Ok(&0));
    assert_eq!(queue, expected_result);
    assert_eq!(queue.dequeue(), Err("Unable to dequeue an empty queue"));
}
