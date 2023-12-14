pub struct Queue<T> {
    items: Vec<T>,
    start: usize,
    size: usize,
    capacity: usize
}

impl<T> Queue<T> {
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;

    static EMPTY_QUEUE: Queue<i8> = Queue {
        items: Vec::new(),
        start: 0,
        size: 0,
        capacity: 1
    };

    static INT_QUEUE: Lazy<Queue<i8>> = Lazy::new(|| Queue {
        items: vec![1, 2, 3, 4],
        start: 0,
        size: 4,
        capacity: 5
    });

    #[test]
    fn test_is_empty() {
        assert_eq!(EMPTY_QUEUE.is_empty(), true);
        assert_eq!(INT_QUEUE.is_empty(), false);
    }
}
