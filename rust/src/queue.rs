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

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
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

    static FULL_INT_QUEUE: Lazy<Queue<i8>> = Lazy::new(|| Queue {
        items: vec![0],
        start: 0,
        size: 1,
        capacity: 1
    });

    #[test]
    fn test_is_empty() {
        assert_eq!(EMPTY_QUEUE.is_empty(), true);
        assert_eq!(FULL_INT_QUEUE.is_empty(), false);
    }

    #[test]
    fn test_is_full() {
        assert_eq!(EMPTY_QUEUE.is_full(), false);
        assert_eq!(FULL_INT_QUEUE.is_full(), true);
    }
}
