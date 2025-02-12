use chrono::{DateTime, Utc};

pub fn format_datetime(datetime: DateTime<Utc>) -> String {
    let days_since = Utc::now().signed_duration_since(datetime).num_days();
    if days_since < 1 {
        format!("{}", datetime.format("%H:%M"))
    } else {
        datetime.format("%d/%m/%Y %H:%M").to_string()
    }
}