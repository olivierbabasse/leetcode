//! <https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let len = s.len();
        let (mut ops, mut carry) = (0, 0);
        for i in (1..len).rev() {
            if (match s.as_bytes()[i] {
                b'1' => 1,
                _ => 0,
            } + carry)
                % 2
                != 0
            {
                ops += 2;
                carry = 1;
            } else {
                ops += 1;
            }
        }
        ops + carry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_steps("1101".into()), 6);
        assert_eq!(Solution::num_steps("10".into()), 1);
        assert_eq!(Solution::num_steps("1".into()), 0);
    }
}
