use chrono::{DateTime, Local};

pub fn format(date: &DateTime<Local>) -> String {
    date.format("%Y-%m-%d %H:%M").to_string()
}

pub fn pretty_diff(from: DateTime<Local>, to: DateTime<Local>) -> String {
    let diff = to - from;

    match diff.num_seconds() {
        s if s < 60 => format!("{} seconds", s),
        s if s < 3600 => format!("{} minutes", s / 60),
        s if s < 86400 => format!("{} hours", s / 3600),
        _ => format!("{} days", diff.num_days()),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Result;

    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

    use crate::utils::time;

    fn local_date_from_string(date: &str) -> DateTime<Local> {
        let naive_datetime =
            NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M:%S").expect("Failed to parse datetime");

        let datetime_local: DateTime<Local> = Local
            .from_local_datetime(&naive_datetime)
            .single()
            .expect("Failed to convert to local datetime");

        datetime_local
    }

    #[test]
    fn format() -> Result<()> {
        let expected = "2024-12-16 15:30".to_string();
        let result = time::format(&local_date_from_string("2024-12-16T15:30:42"));

        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn diff_pretty() -> Result<()> {
        let t4 = local_date_from_string("2024-12-06T15:30:42");
        let t3 = local_date_from_string("2024-12-15T15:31:42");
        let t2 = local_date_from_string("2024-12-16T14:31:42");
        let t1 = local_date_from_string("2024-12-16T15:29:44");
        let t0 = local_date_from_string("2024-12-16T15:30:42");

        assert_eq!("58 seconds", time::pretty_diff(t1, t0));
        assert_eq!("59 minutes", time::pretty_diff(t2, t0));
        assert_eq!("23 hours", time::pretty_diff(t3, t0));
        assert_eq!("10 days", time::pretty_diff(t4, t0));

        Ok(())
    }
}
