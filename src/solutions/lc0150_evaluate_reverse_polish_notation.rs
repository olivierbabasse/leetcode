//! <https://leetcode.com/problems/evaluate-reverse-polish-notation/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack = Vec::new();
        for t in tokens {
            if let Ok(v) = t.parse::<i32>() {
                stack.push(v);
            } else {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                match t.as_ref() {
                    "+" => stack.push(a + b),
                    "-" => stack.push(a - b),
                    "*" => stack.push(a * b),
                    "/" => stack.push(a / b),
                    _ => {}
                }
            }
        }
        stack.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".into(),
                "1".into(),
                "+".into(),
                "3".into(),
                "*".into()
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".into(),
                "13".into(),
                "5".into(),
                "/".into(),
                "+".into()
            ]),
            6
        );
    }
}
