use std::collections::VecDeque;

struct FifoQueue<T> {
    data: VecDeque<T>,
}

impl<T> FifoQueue<T> {
    fn new() -> Self {
        Self {
            data: VecDeque::new(),
        }
    }

    fn new_with_capacity(capacity: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(capacity),
        }
    }

    /// 入队
    fn enqueue(&mut self, element: T) {
        self.data.push_back(element);
    }

    /// 出队
    fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// 队列大小
    fn size(&self) -> usize {
        self.data.len()
    }

    /// 查看队首元素（只读引用）
    fn peek(&self) -> Option<&T> {
        self.data.front()
    }

    /// 查看队首元素（可变引用）
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.data.front_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::queue::FifoQueue;

    #[test]
    fn test_fifo_behavior() {
        let mut queue = FifoQueue::new();
        queue.enqueue("First");
        queue.enqueue("Second");
        queue.enqueue("Third");

        // 验证 FIFO：第一个进去的应该第一个出来
        assert_eq!(queue.dequeue(), Some("First"));
        assert_eq!(queue.dequeue(), Some("Second"));

        // 验证 peek 依然指向当前的“首位”
        assert_eq!(queue.peek(), Some(&"Third"));
    }

    #[test]
    fn test_capacity() {
        let mut queue = FifoQueue::new_with_capacity(10);
        for i in 0..10 {
            queue.enqueue(i);
        }
        assert_eq!(queue.size(), 10);
    }
}
