#[derive(Debug, PartialEq)]
pub struct Queue<T> {
    pub items: Vec<T>,
    pub start: usize,
    pub size: usize,
    pub capacity: usize,
}

impl<T> Queue<T> {
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    pub fn enqueue(&mut self, value: T) -> Result<usize, &str> {
        if self.is_full() {
            Err("Unable to enqueue a full queue")
        } else {
            let index = (self.start + self.size) % self.capacity;
            self.items[index] = value;
            self.size += 1;
            Ok(index)
        }
    }

    pub fn dequeue(&mut self) -> Result<&T, &str> {
        if self.is_empty() {
            Err("Unable to dequeue an empty queue")
        } else {
            let dequeued_item = &self.items[self.start];
            self.start = (self.start + 1) % self.capacity;
            self.size -= 1;
            Ok(dequeued_item)
        }
    }
}
