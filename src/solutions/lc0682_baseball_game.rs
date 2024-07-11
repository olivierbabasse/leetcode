//! <https://leetcode.com/problems/baseball-game/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn cal_points(operations: Vec<String>) -> i32 {
        let mut values = Vec::new();
        for o in operations {
            let len = values.len();
            if o == "+" {
                values.push(values[len - 1] + values[len - 2]);
            } else if o == "D" {
                values.push(values[len - 1] * 2);
            } else if o == "C" {
                values.pop();
            } else {
                values.push(o.parse().unwrap());
            }
        }
        values.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::cal_points(vec![
                "5".into(),
                "2".into(),
                "C".into(),
                "D".into(),
                "+".into()
            ]),
            30
        );
    }
}
