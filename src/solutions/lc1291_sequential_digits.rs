//! <https://leetcode.com/problems/sequential-digits/>

struct Solution {}

/// time-complexity : O(1)
/// space-complexity : O(1)
impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut sd = Vec::new();
        for i in '1'..='9' {
            for j in i..='9' {
                let mut s = String::new();
                for x in i..=j {
                    s.push(x);
                }
                sd.push(s.parse::<i32>().unwrap());
            }
        }
        sd.sort_unstable();
        sd.into_iter().filter(|&p| p >= low && p <= high).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::sequential_digits(100, 300), vec![123, 234]);
        assert_eq!(
            Solution::sequential_digits(1000, 13000),
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]
        );
    }
}
