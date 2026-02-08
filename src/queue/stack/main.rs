/// 栈。如果不指定大小，默认为 1024
struct Stack<T, const N: usize = 1024> {
    data: [Option<T>; N],
    top: usize,
}

#[derive(thiserror::Error, Debug)]
enum StackError {
    #[error("Stack underflow")]
    Underflow,

    #[error("Stack overflow")]
    Overflow,
}

impl<T, const N: usize> Stack<T, N> {
    fn new() -> Self {
        Stack {
            // 通过闭包生成每一项，不要求 T 实现 Copy
            data: std::array::from_fn(|_| None),
            top: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_full(&self) -> bool {
        self.top == N
    }

    fn push(&mut self, item: T) -> Result<(), StackError> {
        if self.is_full() {
            return Err(StackError::Overflow);
        }

        self.data[self.top] = Some(item);
        self.top += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            // 使用 take 取出 Option 中的值，并留下 None，解决了所有权问题
            self.data[self.top].take()
        }
    }

    /// 获取栈顶只读引用（不出栈）
    // fn peek(&self) -> Option<&T> {
    //     self.top
    //         .checked_sub(1)
    //         .and_then(|idx| self.data.get(idx))
    //         .and_then(|opt| opt.as_ref())
    // }

    fn peek(&self) -> Result<&T, StackError> {
        self.top
            .checked_sub(1)
            .and_then(|idx| self.data.get(idx))
            .and_then(|opt| opt.as_ref())
            .ok_or(StackError::Underflow)
    }

    /// 获取栈顶可变引用（不出栈）
    // fn peek_mut(&mut self) -> Option<&mut T> {
    //     self.top
    //         .checked_sub(1)
    //         .and_then(|idx| self.data.get_mut(idx))
    //         .and_then(|opt| opt.as_mut())
    // }

    fn peek_mut(&mut self) -> Result<&mut T, StackError> {
        self.top
            .checked_sub(1)
            .and_then(|idx| self.data.get_mut(idx))
            .and_then(|opt| opt.as_mut())
            .ok_or(StackError::Underflow)
    }
}

fn main() {
    // todo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_and_pop() {
        let mut stack: Stack<i32, 2> = Stack::new();
        assert!(stack.is_empty());

        stack.push(1).unwrap();
        stack.push(2).unwrap();
        assert!(stack.is_full());

        // 测试溢出
        assert!(matches!(stack.push(3), Err(StackError::Overflow)));

        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<String, 5> = Stack::new();
        stack.push("Rust".to_string()).unwrap();
        stack.push("Go".to_string()).unwrap();

        // 只读查看
        assert_eq!(stack.peek().unwrap(), "Go");

        // 可变修改
        if let Ok(top_val) = stack.peek_mut() {
            top_val.push_str("lang");
        }

        assert_eq!(stack.peek().unwrap(), "Golang");
        assert_eq!(stack.pop(), Some("Golang".to_string()));
    }

    #[test]
    fn test_empty_errors() {
        let mut stack: Stack<i32, 10> = Stack::new();

        // 尝试对空栈进行 peek
        assert!(matches!(stack.peek(), Err(StackError::Underflow)));
        assert!(matches!(stack.peek_mut(), Err(StackError::Underflow)));
    }

    #[test]
    fn test_default_capacity() {
        // 测试默认常量参数 (N=1024)
        let stack: Stack<u8> = Stack::new();
        assert_eq!(stack.data.len(), 1024);
    }
}
