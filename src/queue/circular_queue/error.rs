use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CircularQueueError {
    /// 队列已满
    #[error("queue is full (capacity: {0})")]
    Full(usize),

    /// 空队列
    #[error("queue is empty")]
    Empty,

    /// 无效的容量，队列容量必须大于 0
    #[error("cannot allocate queue with capacity {0}: must be greater than 0")]
    InvalidCapacity(usize),
}
