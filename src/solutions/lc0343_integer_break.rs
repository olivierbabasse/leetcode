//! <https://leetcode.com/problems/integer-break/>

struct Solution1 {}

impl Solution1 {
    /// time-complexity : O(n^2)
    /// space-complexity : O(n)
    fn find_combinations(
        target: i32,
        elements: &mut Vec<i32>,
        current_sum: i32,
        current_product: i32,
        max_product: &mut i32,
        current: i32,
    ) {
        if current_sum == target {
            if current_product > *max_product {
                *max_product = current_product;
            }
            return;
        }

        if current_sum < target {
            for n in current..target {
                elements.push(n);
                Self::find_combinations(
                    target,
                    elements,
                    current_sum + n,
                    current_product * n,
                    max_product,
                    n,
                );
                elements.pop();
            }
        }
    }

    pub fn integer_break(n: i32) -> i32 {
        let mut elements = Vec::new();
        let mut max_product = 0;
        Self::find_combinations(n, &mut elements, 0, 1, &mut max_product, 1);
        max_product
    }
}

struct Solution2 {}

impl Solution2 {
    /// time-complexity : O(1)
    /// space-complexity : O(1)
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let count = (n / 3) as u32;
        match n % 3 {
            0 => 3i32.pow(count),
            1 => 3i32.pow(count - 1) * 4,
            _ => 3i32.pow(count) * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution1, Solution2};

    #[test]
    fn test_cases_1() {
        assert_eq!(Solution1::integer_break(2), 1);
        assert_eq!(Solution1::integer_break(10), 36);
    }

    #[test]
    fn test_cases_2() {
        assert_eq!(Solution2::integer_break(2), 1);
        assert_eq!(Solution2::integer_break(10), 36);
    }
}
