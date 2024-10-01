//! <https://leetcode.com/problems/check-if-array-pairs-are-divisible-by-k/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(k)
impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut rems = vec![0; k as usize];

        for a in &arr {
            rems[((a % k + k) % k) as usize] += 1;
        }

        if rems[0] % 2 != 0 {
            return false;
        }

        if (1..(k as usize)).any(|i| rems[i] != rems[k as usize - i]) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert!(Solution::can_arrange(
            vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9],
            5
        ));
        assert!(Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 7));
        assert!(!Solution::can_arrange(vec![1, 2, 3, 4, 5, 6], 10));
    }
}
