//! <https://leetcode.com/problems/my-calendar-iii/>

use std::collections::BTreeMap;

struct MyCalendarThree {
    bookings: BTreeMap<i32, i32>,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            bookings: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        *self.bookings.entry(start_time).or_default() += 1;
        *self.bookings.entry(end_time).or_default() -= 1;

        let (mut max, mut sum) = (0, 0);
        for &v in self.bookings.values() {
            sum += v;
            max = max.max(sum);
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cases() {
        let mut my_calendar_three = MyCalendarThree::new();
        assert_eq!(my_calendar_three.book(10, 20), 1);
        assert_eq!(my_calendar_three.book(50, 60), 1);
        assert_eq!(my_calendar_three.book(10, 40), 2);
        assert_eq!(my_calendar_three.book(5, 15), 3);
        assert_eq!(my_calendar_three.book(5, 10), 3);
        assert_eq!(my_calendar_three.book(25, 55), 3);
    }
}
