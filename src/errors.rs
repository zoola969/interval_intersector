use std::fmt::{Display, Error};

#[derive(Debug)]
pub struct IntervalsDontIntersectError;

impl Display for IntervalsDontIntersectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), Error> {
        write!(f, "Intervals don't intersect")
    }
}
