#[derive(Debug, PartialEq)]
pub struct Queue<T> {
    items: Vec<T>,
    start: usize,
    size: usize,
    capacity: usize,
}

impl<T> Queue<T> {
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, value: T) -> Result<usize, &str> {
        if self.is_full() {Err("Unable to enqueue a full queue")}
        else {
            let index = (self.start + self.size) % self.capacity;
            self.items[index] = value;
            self.size += 1;
            return Ok(index);
        }
    }

    pub fn dequeue(&mut self) -> Result<&T, &str> {
        if self.is_empty() {Err("Unable to dequeue an empty queue")}
        else {
            let dequeued_item = &self.items[self.start];
            self.start = (self.start + 1) % self.capacity;
            self.size -= 1;
            return Ok(dequeued_item);
        }
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
        assert_eq!(EMPTY_QUEUE.is_empty(), true);
        assert_eq!(FULL_INT_QUEUE.is_empty(), false);
    }

    #[test]
    fn test_is_full() {
        assert_eq!(EMPTY_QUEUE.is_full(), false);
        assert_eq!(FULL_INT_QUEUE.is_full(), true);
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
}
