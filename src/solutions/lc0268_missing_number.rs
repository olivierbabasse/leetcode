//! <https://leetcode.com/problems/missing-number/>

struct Solution1 {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution1 {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        nums.sort_unstable();
        for (i, num) in nums.into_iter().enumerate() {
            if num != i as i32 {
                return i as i32;
            }
        }
        n as i32
    }
}

struct Solution2 {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution2 {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        ((n * (n + 1)) / 2) as i32 - nums.into_iter().sum::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases_1() {
        assert_eq!(Solution1::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution1::missing_number(vec![0, 1]), 2);
        assert_eq!(
            Solution1::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(Solution2::missing_number(vec![3, 0, 1]), 2);
        assert_eq!(Solution2::missing_number(vec![0, 1]), 2);
        assert_eq!(
            Solution2::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
            8
        );
    }
}
