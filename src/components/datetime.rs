//! Daytime module
//! The function [`datetime`] provides only the current date

/// Get the current date of the local timezone
pub fn datetime() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}
