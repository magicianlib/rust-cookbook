use crate::error::CircularQueueError;
use std::fmt::Debug;

pub struct CircularQueue<T> {
    /// 使用 [Vec] 模拟循环队列
    ///
    /// 使用 [Option] 包装 `T` 可以解决元素所有权转移问题。在不使用 `unsafe` 的情况下，无法直接移除 [Vec] 某个
    /// 索引的值。但是可以利用 [Option::take] 实现，[take](Option::take) 内部通过 [std::mem::replace] 实现，
    /// 可以安全的置换出 [Some] 数据。取走数据所有权之后，原 [Option] 会置为 [None]
    data: Vec<Option<T>>,

    /// 队列大小为实际容量 + 1，用于区分队满和队空
    capacity: usize,

    /// 队列首元素指针
    front: usize,

    /// 当队列 `rear` 指针正好在 `front` 前一个位置时被视为队列已满。
    /// 当 `rear` 指针等于 `front` 时表示队列当前为空。
    rear: usize,
}

impl<T: Debug> CircularQueue<T> {
    /// 使用 `capacity` 初始化队列容量（`capacity` 必须大于0）
    ///
    /// 队列实际容量为：`capacity` + 1，用于区分队满和队空
    pub fn build(capacity: usize) -> Result<Self, CircularQueueError> {
        if capacity == 0 {
            return Err(CircularQueueError::InvalidCapacity(0));
        }

        // 预先填充 None，后续直接使用下标取数据
        let actual_capacity = capacity + 1;
        let mut data = Vec::with_capacity(actual_capacity);
        for _ in 0..actual_capacity {
            data.push(None);
        }

        Ok(Self {
            data,
            capacity: actual_capacity,
            front: 0,
            rear: 0,
        })
    }

    /// 当队列 `rear` 指针正好在 `front` 前一个位置时被视为队列已满
    pub fn is_full(&self) -> bool {
        (self.rear + 1) % self.capacity == self.front
    }

    /// 当 `rear` 指针等于 `front` 时表示队列当前为空
    pub fn is_empty(&self) -> bool {
        self.rear == self.front
    }

    /// 入队
    pub fn enqueue(&mut self, element: T) -> Result<(), CircularQueueError> {
        if self.is_full() {
            // 这里返回 capacity - 1 是因为实际容量多占了一位
            Err(CircularQueueError::Full(self.capacity - 1))
        } else {
            // 使用 push 会导致 Vec 不断增加长度，所以这里必须使用下标
            self.data[self.rear] = Some(element);

            // 移动 rear 位置，新位置可能超过队列最大容量（越界）。
            // 通过对容量取余运算，可以实现 rear 回到队列的起点，达到回环效果
            self.rear = (self.rear + 1) % self.capacity;

            Ok(())
        }
    }

    /// 出队
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            // 使用 Option::take 取走数据所有权，取走所有权之后原 Option 会被设置为 None
            let element = self.data[self.front].take();

            // front 与 rear 一样，同样可能存在越界情况。
            // 所以也需要对容量做取余运算，使 front 回到数组起点
            self.front = (self.front + 1) % self.capacity;

            element
        }
    }

    /// 查看队首元素（只读引用）
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            // as_ref 将 Option<T> 转换为 Option<&T>
            self.data[self.front].as_ref()
        }
    }

    /// 查看队首元素（可变引用）
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data[self.front].as_mut()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::CircularQueueError;
    use crate::queue::CircularQueue;

    #[test]
    fn test_build_invalid_capacity() {
        let queue = CircularQueue::<i32>::build(0);
        assert!(matches!(queue, Err(CircularQueueError::InvalidCapacity(_))));
    }

    #[test]
    fn test_enqueue_and_dequeue() {
        let mut queue = CircularQueue::build(2).unwrap();

        // 初始状态
        assert!(queue.is_empty());

        // 入队
        queue.enqueue(10).unwrap();
        queue.enqueue(20).unwrap();
        assert!(queue.is_full());

        // 验证 peak
        assert_eq!(queue.peek(), Some(&10));

        // 出队
        assert_eq!(queue.dequeue(), Some(10));
        assert_eq!(queue.peek(), Some(&20));
        assert_eq!(queue.dequeue(), Some(20));

        // 队列为空
        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_queue_full_error() {
        let mut queue = CircularQueue::build(1).unwrap();
        queue.enqueue(1).unwrap();

        // 尝试向已满的队列插入数据
        let result = queue.enqueue(2);
        assert!(matches!(result, Err(CircularQueueError::Full(_))));
    }

    #[test]
    fn test_circular_behavior() {
        // 验证索引环绕逻辑
        let mut queue = CircularQueue::build(2).unwrap();

        queue.enqueue(1).unwrap();
        queue.dequeue(); // front 移动

        queue.enqueue(2).unwrap();
        queue.enqueue(3).unwrap(); // rear 环绕

        assert!(queue.is_full());
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
    }

    #[test]
    fn test_peek_mut() {
        let mut queue = CircularQueue::build(1).unwrap();
        queue.enqueue(10).unwrap();

        // 通过 peek_mut 修改值
        if let Some(element) = queue.peek_mut() {
            *element = 50;
        }

        assert_eq!(queue.dequeue(), Some(50));
    }
}
