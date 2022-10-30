#[cfg(test)]
pub mod test_utils {
    use chrono::{DateTime, Utc};

    pub fn build_utc_dt(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        return DateTime::from_utc(
            chrono::NaiveDate::from_ymd(year, month, day).and_hms(0, 0, 0),
            Utc,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::test_utils::build_utc_dt;
    use chrono::{Datelike, Utc};

    #[test]
    fn test_build_dt() {
        let year = 2022;
        let month = 01;
        let day = 01;
        let dt = build_utc_dt(year, month, day);
        assert_eq!(dt.year(), year);
        assert_eq!(dt.month(), month);
        assert_eq!(dt.day(), day);
        assert_eq!(dt.timezone(), Utc)
    }
}
