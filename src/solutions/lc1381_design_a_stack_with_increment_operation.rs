//! <https://leetcode.com/problems/design-a-stack-with-increment-operation/>

struct CustomStack {
    stack: Vec<i32>,
    max_size: usize,
}

/// time-complexity : push/pop O(1) increment O(n)
/// space-complexity : O(n)
impl CustomStack {
    fn new(max_size: i32) -> Self {
        Self {
            stack: Vec::new(),
            max_size: max_size as usize,
        }
    }

    fn push(&mut self, x: i32) {
        if self.stack.len() < self.max_size {
            self.stack.push(x);
        }
    }

    fn pop(&mut self) -> i32 {
        self.stack.pop().unwrap_or(-1)
    }

    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..(k as usize).min(self.stack.len()) {
            self.stack[i] += val;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut stack = CustomStack::new(3);
        stack.push(1);
        stack.push(2);
        stack.pop();
        stack.push(2);
        stack.push(3);
        stack.push(4);
        stack.increment(5, 100);
        stack.increment(2, 100);
        assert_eq!(stack.pop(), 103);
        assert_eq!(stack.pop(), 202);
        assert_eq!(stack.pop(), 201);
        assert_eq!(stack.pop(), -1);
    }
}
