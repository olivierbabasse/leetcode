//! <https://leetcode.com/problems/integer-break/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let count = (n / 3) as u32;
        match n % 3 {
            0 => 3i32.pow(count),
            1 => 3i32.pow(count - 1) * 4,
            _ => 3i32.pow(count) * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases_2() {
        assert_eq!(Solution::integer_break(2), 1);
        assert_eq!(Solution::integer_break(10), 36);
    }
}
