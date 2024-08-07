//! <https://leetcode.com/problems/integer-to-english-words/>

struct Solution {}

/// time-complexity : O(log(n))
/// space-complexity : O(log(n))
impl Solution {
    const UNITS: [&'static str; 20] = [
        "",
        "One",
        "Two",
        "Three",
        "Four",
        "Five",
        "Six",
        "Seven",
        "Eight",
        "Nine",
        "Ten",
        "Eleven",
        "Twelve",
        "Thirteen",
        "Fourteen",
        "Fifteen",
        "Sixteen",
        "Seventeen",
        "Eighteen",
        "Nineteen",
    ];
    const TENS: [&'static str; 10] = [
        "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
    ];
    const THOUSANDS: [&'static str; 4] = ["", "Thousand", "Million", "Billion"];

    fn handle_group(num: i32) -> String {
        if num == 0 {
            "".to_string()
        } else if num < 20 {
            Self::UNITS[num as usize].to_string()
        } else if num < 100 {
            let mut result = Self::TENS[(num / 10) as usize].to_string();
            if num % 10 != 0 {
                result.push_str(&format!(" {}", Self::UNITS[(num % 10) as usize]));
            }
            result
        } else {
            let mut result = Self::UNITS[(num / 100) as usize].to_string();
            result.push_str(" Hundred");
            if num % 100 != 0 {
                result.push_str(&format!(" {}", &Self::handle_group(num % 100)));
            }
            result
        }
    }

    pub fn number_to_words(mut num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }

        let mut result = String::new();
        let mut i = 0;

        while num > 0 {
            if num % 1000 != 0 {
                let chunk = Self::handle_group(num % 1000);
                if !result.is_empty() {
                    result = format!("{} {} {}", chunk, Self::THOUSANDS[i], result);
                } else {
                    result = format!("{} {}", chunk, Self::THOUSANDS[i]);
                }
            }
            num /= 1000;
            i += 1;
        }

        result.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        assert_eq!(
            Solution::number_to_words(123),
            "One Hundred Twenty Three".to_string()
        );
        assert_eq!(
            Solution::number_to_words(12345),
            "Twelve Thousand Three Hundred Forty Five".to_string()
        );
        assert_eq!(
            Solution::number_to_words(1234567),
            "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven".to_string()
        );
    }
}
