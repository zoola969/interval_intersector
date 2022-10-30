use crate::errors::IntervalsDontIntersectError;
use chrono::{DateTime, Utc};
use std::cmp::{max, min};

#[derive(Copy, Clone, PartialEq, Debug, Eq)]
pub struct Interval {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>, // end always exclusive
}

impl Interval {
    pub fn is_intersects_with(&self, other: &Interval) -> bool {
        if self.end <= other.start {
            return false;
        }
        if other.end <= self.start {
            return false;
        }
        return true;
    }

    pub fn intersects_with(
        &self,
        other: &Interval,
    ) -> Result<Interval, IntervalsDontIntersectError> {
        if !self.is_intersects_with(other) {
            return Err(IntervalsDontIntersectError);
        }
        return Ok(Interval {
            start: max(self.start, other.start),
            end: min(self.end, other.end),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::Interval;
    use crate::test_utils::test_utils::build_utc_dt;

    #[test]
    fn test_is_interval_intersects_gap() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 03),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 03),
            end: build_utc_dt(2022, 01, 05),
        };
        assert!(!&interval_1.is_intersects_with(&interval_2));
    }

    #[test]
    fn test_is_interval_intersects_overlapping() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 04),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 03),
            end: build_utc_dt(2022, 01, 05),
        };
        assert!(&interval_1.is_intersects_with(&interval_2));
    }
    #[test]
    fn test_is_interval_intersects_content() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 10),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 05),
        };
        assert!(&interval_1.is_intersects_with(&interval_2));
    }
    #[test]
    fn test_intersects_intervals_content() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 10),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 05),
        };
        assert_eq!(
            interval_1.intersects_with(&interval_2).unwrap(),
            Interval {
                start: build_utc_dt(2022, 01, 01),
                end: build_utc_dt(2022, 01, 05),
            }
        );
    }
    #[test]
    fn test_intersects_intervals_gap() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 03),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 03),
            end: build_utc_dt(2022, 01, 05),
        };
        assert!(interval_1.intersects_with(&interval_2).is_err());
    }
    #[test]
    fn test_intersects_intervals_overlapping() {
        let interval_1 = Interval {
            start: build_utc_dt(2022, 01, 01),
            end: build_utc_dt(2022, 01, 04),
        };
        let interval_2 = Interval {
            start: build_utc_dt(2022, 01, 03),
            end: build_utc_dt(2022, 01, 05),
        };
        assert_eq!(
            interval_1.intersects_with(&interval_2).unwrap(),
            Interval {
                start: build_utc_dt(2022, 01, 03),
                end: build_utc_dt(2022, 01, 04),
            }
        );
    }
}
