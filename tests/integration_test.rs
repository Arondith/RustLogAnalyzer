use rust_log_analyzer::{filter_by_level, parse_contents, LogLevel};

#[test]
fn filters_error_entries_from_multiple_lines() {
    let logs = "[2026-09-18T09:00:00Z] [INFO] Server started\n\
                [2026-09-18T09:01:00Z] [ERROR] Timeout\n\
                [2026-09-18T09:02:00Z] [ERROR] Database unavailable";

    let entries = parse_contents(logs);
    let errors = filter_by_level(&entries, &LogLevel::Error);

    assert_eq!(errors.len(), 2);
    assert_eq!(errors[0].message, "Timeout");
    assert_eq!(errors[1].message, "Database unavailable");
}
