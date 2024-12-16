use chrono::{DateTime, Local};

pub fn format(date: DateTime<Local>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    use crate::time;

    fn local_date_from_string(date: &str) -> DateTime<Local> {
        let naive_datetime = NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S")
            .expect("Failed to parse datetime");

        let datetime_local: DateTime<Local> = Local.from_local_datetime(&naive_datetime)
            .single()
            .expect("Failed to convert to local datetime");

        datetime_local
    }

    #[test]
    fn format() -> Result<()> {
        let expected = "2024-12-16 15:30".to_string();
        let result = time::format(local_date_from_string("2024-12-16T15:30:42"));

        assert_eq!(expected, result);

        Ok(())
    }
}
