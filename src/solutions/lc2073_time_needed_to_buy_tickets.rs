//! <https://leetcode.com/problems/time-needed-to-buy-tickets/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(1)
impl Solution {
    pub fn time_required_to_buy(mut tickets: Vec<i32>, k: i32) -> i32 {
        let mut t = 0;
        let mut index = 0;
        loop {
            let pos = index % tickets.len();
            if tickets[pos] > 0 {
                tickets[pos] -= 1;
                t += 1;
            }
            if pos == k as usize && tickets[pos] == 0 {
                break;
            }
            index += 1;
        }
        t
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
