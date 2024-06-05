//! <https://leetcode.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(1)
impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut prefixes = vec![0; arr.len() + 1];
        prefixes[1..].clone_from_slice(&arr);
        let len = prefixes.len();
        let mut count = 0;

        for i in 1..len {
            prefixes[i] ^= prefixes[i - 1];
        }

        for begin in 0..len {
            for end in begin + 1..len {
                if prefixes[begin] == prefixes[end] {
                    count += end - begin - 1;
                }
            }
        }

        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
        assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
    }
}
