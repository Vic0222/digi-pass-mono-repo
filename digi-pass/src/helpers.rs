#[cfg(not(test))]
pub fn get_current_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}


#[cfg(any(test))]
pub fn get_current_time() -> chrono::DateTime<chrono::Utc> {
    match chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2010, 1, 1, 1, 1, 1) {
        chrono::LocalResult::None => chrono::Utc::now(),
        chrono::LocalResult::Single(time) => time,
        chrono::LocalResult::Ambiguous(time, _) => time,
    }

}


