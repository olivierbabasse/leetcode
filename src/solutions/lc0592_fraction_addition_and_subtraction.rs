//! <https://leetcode.com/problems/fraction-addition-and-subtraction/>

struct Solution {}

/// time-complexity : O(n)
/// space-complexity : O(n)
impl Solution {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    pub fn fraction_addition(mut expression: String) -> String {
        if expression.as_bytes()[0] != b'-' {
            expression.insert(0, '+');
        }

        let mut fracs = Vec::new();
        let mut i = 0;
        while i < expression.len() {
            let mut j = i + 1;
            while j < expression.len() && ![b'+', b'-'].contains(&expression.as_bytes()[j]) {
                j += 1;
            }
            fracs.push(&expression[i..j]);
            i = j;
        }

        let (mut num, mut den) = (0, 1);
        for f in fracs {
            let parts = f.split('/').collect::<Vec<_>>();
            let n = parts[0].parse::<i64>().unwrap();
            let d = parts[1].parse::<i64>().unwrap();
            num = num * d + n * den;
            den *= d;
            let gcd = Self::gcd(num, den);
            dbg!((num, den, gcd));
            num /= gcd;
            den /= gcd;
        }

        format!("{num}/{den}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2".into()),
            "0/1".to_string()
        );
        assert_eq!(
            Solution::fraction_addition("-1/2+1/2+1/3".into()),
            "1/3".to_string()
        );
        assert_eq!(
            Solution::fraction_addition("1/3-1/2".into()),
            "-1/6".to_string()
        );
    }
}
