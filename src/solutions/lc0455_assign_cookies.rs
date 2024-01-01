//! <https://leetcode.com/problems/assign-cookies/>

struct Solution {}

/// time-complexity : O(n*log(n))
/// space-complexity : O(1)
impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort_unstable();
        s.sort_unstable();
        let mut count = 0;
        let mut pos = 0;
        for c in g {
            if let Some(index) = s.iter().skip(pos).position(|&e| e >= c) {
                s[pos + index] = 0;
                pos += index + 1;
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::find_content_children(vec![1, 2, 3], vec![1, 1]),
            1
        );
        assert_eq!(
            Solution::find_content_children(vec![1, 2], vec![1, 2, 3]),
            2
        );
        assert_eq!(
            Solution::find_content_children(vec![10, 9, 8, 7, 10, 9, 8, 7], vec![10, 9, 8, 7]),
            4
        );
    }
}
