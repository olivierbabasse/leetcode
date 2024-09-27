//! <https://leetcode.com/problems/my-calendar-ii/>

use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendarTwo {
    bookings: BTreeMap<i32, i32>,
}

/// time-complexity : O(log(n))
/// space-complexity : O(n)
impl MyCalendarTwo {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        *self.bookings.entry(start).or_default() += 1;
        *self.bookings.entry(end).or_default() -= 1;

        let (mut max, mut sum) = (0, 0);
        for &v in self.bookings.values() {
            sum += v;
            max = max.max(sum);
        }

        if max > 2 {
            self.bookings.entry(start).and_modify(|v| *v -= 1);
            self.bookings.entry(end).and_modify(|v| *v += 1);
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut my_calendar_two = MyCalendarTwo::new();
        assert!(my_calendar_two.book(10, 20));
        assert!(my_calendar_two.book(50, 60));
        assert!(my_calendar_two.book(10, 40));
        assert!(!my_calendar_two.book(5, 15));
        assert!(my_calendar_two.book(5, 10));
        assert!(my_calendar_two.book(25, 55));
    }
}
