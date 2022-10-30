use crate::{interval::Interval, interval_sequence::IntervalsSequence};
use log::debug;

pub fn intersects_intervals_sequences(sequences: &Vec<IntervalsSequence>) -> IntervalsSequence {
    if sequences.len() == 0 {
        return IntervalsSequence::new(Vec::new());
    }
    if sequences.len() == 1 {
        return sequences[0].clone();
    }
    let mut result_sequence = sequences[0].get_intervals().clone();
    debug!("Initial sequence: {:?}", result_sequence);
    for sequence in &sequences[1..] {
        debug!("Next sequence: {:?}", sequence);
        let mut result_intervals: Vec<Interval> = Vec::new();
        for first_interval in &result_sequence {
            debug!("First interval: {:?}", first_interval);
            for second_interval in sequence.get_intervals() {
                debug!("Second interval: {:?}", second_interval);
                if second_interval.end < first_interval.start {
                    debug!(
                        "Second interval is before first interval, moving to next second interval"
                    );
                    continue;
                }
                if second_interval.start > first_interval.end {
                    debug!(
                        "Second interval is after first interval, moving to next first interval"
                    );
                    break;
                }
                if first_interval.is_intersects_with(second_interval) {
                    debug!("Second interval intersects first interval");
                    let new_interval = first_interval.intersects_with(second_interval).unwrap();
                    debug!("New interval: {:?}", new_interval);
                    result_intervals.push(new_interval);
                }
            }
        }
        result_sequence = result_intervals;
    }
    return IntervalsSequence::new(result_sequence);
}

#[cfg(test)]
mod tests {
    use super::intersects_intervals_sequences;
    use crate::test_utils::test_utils::build_utc_dt;
    use crate::{interval::Interval, interval_sequence::IntervalsSequence};

    #[test]
    fn test_intersects_intervals_sequences_empty() {
        let sequences: Vec<IntervalsSequence> = Vec::new();
        let result = intersects_intervals_sequences(&sequences);
        assert_eq!(result.get_intervals().len(), 0);
    }

    #[test]
    fn test_intersects_intervals_sequences_one_seq() {
        let seq = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 03),
            },
            Interval {
                start: build_utc_dt(2000, 01, 03),
                end: build_utc_dt(2000, 01, 05),
            },
        ]);
        let expected_seq = seq.clone();
        let sequences = vec![seq];

        let result = intersects_intervals_sequences(&sequences);
        assert_eq!(result, expected_seq);
    }

    #[test]
    fn test_intersects_intervals_sequences_identical() {
        let seq_1 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 10),
            },
            Interval {
                start: build_utc_dt(2000, 01, 20),
                end: build_utc_dt(2000, 01, 30),
            },
        ]);
        let seq_2 = seq_1.clone();
        let seq_3 = seq_1.clone();
        let expected_seq = seq_1.clone();

        let result = intersects_intervals_sequences(&vec![seq_1, seq_2, seq_3]);
        assert_eq!(result, expected_seq,);
    }

    #[test]
    fn test_intersects_intervals_sequence_one_of_is_empty() {
        let seq_1 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 10),
            },
            Interval {
                start: build_utc_dt(2000, 01, 20),
                end: build_utc_dt(2000, 01, 30),
            },
        ]);
        let seq_2 = IntervalsSequence::new(Vec::new());
        let seq_3 = seq_1.clone();
        let expected_seq = seq_2.clone();

        let result = intersects_intervals_sequences(&vec![seq_1, seq_2, seq_3]);
        assert_eq!(result, expected_seq);
    }

    #[test]
    fn test_intersects_intervals_sequences_no_intersection() {
        let seq_1 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 10),
            },
            Interval {
                start: build_utc_dt(2000, 01, 20),
                end: build_utc_dt(2000, 01, 30),
            },
        ]);
        let seq_2 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 10),
                end: build_utc_dt(2000, 01, 10),
            },
            Interval {
                start: build_utc_dt(2000, 02, 01),
                end: build_utc_dt(2000, 02, 10),
            },
        ]);
        let seq_3 = IntervalsSequence::new(vec![
            seq_1.get_intervals()[0].clone(),
            seq_2.get_intervals()[1].clone(),
        ]);

        let expected_seq = IntervalsSequence::new(Vec::new());

        let result = intersects_intervals_sequences(&vec![seq_1, seq_2, seq_3]);
        assert_eq!(result, expected_seq);
    }
    // 01-05 10-17 20-25
    // 01-15 16-18 21-30
    // 02-10 15-30
    // 02-05 16-17 21-25
    #[test]
    fn test_intersects_intervals_sequences() {
        let seq_1 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 05),
            },
            Interval {
                start: build_utc_dt(2000, 01, 10),
                end: build_utc_dt(2000, 01, 17),
            },
            Interval {
                start: build_utc_dt(2000, 01, 20),
                end: build_utc_dt(2000, 01, 25),
            },
        ]);
        let seq_2 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 01),
                end: build_utc_dt(2000, 01, 15),
            },
            Interval {
                start: build_utc_dt(2000, 01, 16),
                end: build_utc_dt(2000, 01, 18),
            },
            Interval {
                start: build_utc_dt(2000, 01, 21),
                end: build_utc_dt(2000, 01, 30),
            },
        ]);
        let seq_3 = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 02),
                end: build_utc_dt(2000, 01, 10),
            },
            Interval {
                start: build_utc_dt(2000, 01, 15),
                end: build_utc_dt(2000, 01, 30),
            },
        ]);

        let expected_seq = IntervalsSequence::new(vec![
            Interval {
                start: build_utc_dt(2000, 01, 02),
                end: build_utc_dt(2000, 01, 05),
            },
            Interval {
                start: build_utc_dt(2000, 01, 16),
                end: build_utc_dt(2000, 01, 17),
            },
            Interval {
                start: build_utc_dt(2000, 01, 21),
                end: build_utc_dt(2000, 01, 25),
            },
        ]);

        let result = intersects_intervals_sequences(&vec![seq_1, seq_2, seq_3]);
        assert_eq!(result, expected_seq);
    }
}
