//! <https://leetcode.com/problems/perfect-squares/>

struct Solution {}

/// time-complexity : O()
/// space-complexity : O()
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut squares = Vec::new();
        let mut i = 1;
        while i * i <= n {
            if i * i == n {
                return 1;
            }
            squares.push(i * i);
            i += 1;
        }
        let len = squares.len();

        for i in 0..len {
            for j in i..len {
                if squares[i] + squares[j] == n {
                    return 2;
                }
            }
        }

        for i in 0..len {
            for j in i..len {
                for k in j..len {
                    if squares[i] + squares[j] + squares[k] == n {
                        return 3;
                    }
                }
            }
        }

        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
