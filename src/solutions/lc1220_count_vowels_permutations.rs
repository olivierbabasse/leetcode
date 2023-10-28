//! <https://leetcode.com/problems/count-vowels-permutation/>

struct Solution {}

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        const MODULO: i64 = 1000000007;

        let mut a = 1;
        let mut e = 1;
        let mut i = 1;
        let mut o = 1;
        let mut u = 1;

        for _ in 1..n {
            let next_a = e;
            let next_e = (a + i) % MODULO;
            let next_i = (a + e + o + u) % MODULO;
            let next_o = (i + u) % MODULO;
            let next_u = a;

            a = next_a;
            e = next_e;
            i = next_i;
            o = next_o;
            u = next_u;
        }

        ((a + e + i + o + u) % MODULO) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(Solution::count_vowel_permutation(1), 5);
        assert_eq!(Solution::count_vowel_permutation(2), 10);
        assert_eq!(Solution::count_vowel_permutation(5), 68);
    }
}
