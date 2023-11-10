//! <https://leetcode.com/problems/calculate-amount-paid-in-taxes/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut income = income as f64;
        let mut tax = 0.0;
        let mut last_bracket = 0.0;
        for b in brackets {
            let taxed = income.min(b[0] as f64 - last_bracket);
            last_bracket = b[0] as f64;
            tax += taxed * b[1] as f64 / 100.0;
            income -= taxed;
            if income <= 0.0 {
                break;
            }
        }
        tax
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::calculate_tax(vec![vec![3, 50], vec![7, 10], vec![12, 25]], 10),
            2.65
        );
        assert_eq!(
            Solution::calculate_tax(vec![vec![1, 0], vec![4, 25], vec![5, 50]], 2),
            0.25
        );
        assert_eq!(Solution::calculate_tax(vec![vec![2, 50]], 0), 0.0);
    }
}
