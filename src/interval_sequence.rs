use crate::interval::Interval;

#[derive(Clone, PartialEq, Debug, Eq)]
pub struct IntervalsSequence {
    intervals: Vec<Interval>,
}

impl IntervalsSequence {
    pub fn new(intervals: Vec<Interval>) -> IntervalsSequence {
        let mut sorted_intervals = intervals.clone();
        sorted_intervals.sort_by(|a, b| a.start.cmp(&b.start));
        return IntervalsSequence {
            intervals: sorted_intervals,
        };
    }

    pub fn get_intervals(&self) -> &Vec<Interval> {
        return &self.intervals;
    }
}
