//! <https://leetcode.com/problems/couples-holding-hands/>

struct Solution {}

/// time-complexity : O(n^2)
/// space-complexity : O(1)
impl Solution {
    pub fn min_swaps_couples(mut row: Vec<i32>) -> i32 {
        let len = row.len();
        let mut res = 0;
        for i in (0..len).step_by(2) {
            let partner = if row[i] % 2 == 0 {
                row[i] + 1
            } else {
                row[i] - 1
            };
            if row[i + 1] != partner {
                res += 1;
                for j in i + 1..len {
                    if row[j] == partner {
                        row.swap(i + 1, j);
                        break;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::min_swaps_couples(vec![0, 2, 1, 3]), 1);
        assert_eq!(Solution::min_swaps_couples(vec![3, 2, 0, 1]), 0);
    }
}
