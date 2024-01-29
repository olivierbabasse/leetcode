//! <https://leetcode.com/problems/implement-queue-using-stacks/>

#[derive(Default)]
struct MyQueue {
    front: Vec<i32>,
    back: Vec<i32>,
}

/// time-complexity : amortized O(1)
/// space-complexity : O(n)
impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn transfer(&mut self) {
        if self.front.is_empty() {
            while let Some(i) = self.back.pop() {
                self.front.push(i);
            }
        }
    }

    fn push(&mut self, x: i32) {
        self.back.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.transfer();
        self.front.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        self.transfer();
        *self.front.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.front.is_empty() && self.back.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut obj = MyQueue::new();
        obj.push(1);
        obj.push(2);
        assert_eq!(obj.peek(), 1);
        assert_eq!(obj.pop(), 1);
        assert!(!obj.empty());
    }
}
