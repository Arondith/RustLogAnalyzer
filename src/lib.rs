use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Unknown(String),
}

impl LogLevel {
    pub fn parse(value: &str) -> Self {
        match value.trim().to_ascii_uppercase().as_str() {
            "TRACE" => Self::Trace,
            "DEBUG" => Self::Debug,
            "INFO" => Self::Info,
            "WARN" | "WARNING" => Self::Warn,
            "ERROR" => Self::Error,
            other => Self::Unknown(other.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Unknown(value) => value.as_str(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub timestamp: Option<String>,
    pub level: LogLevel,
    pub message: String,
}

pub fn parse_line(line: &str) -> Option<LogEntry> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    if let Some(after_timestamp) = line.strip_prefix('[') {
        if let Some(timestamp_end) = after_timestamp.find(']') {
            let timestamp = after_timestamp[..timestamp_end].trim();
            let remaining = after_timestamp[timestamp_end + 1..].trim_start();

            if let Some(after_level) = remaining.strip_prefix('[') {
                if let Some(level_end) = after_level.find(']') {
                    let level = LogLevel::parse(&after_level[..level_end]);
                    let message = after_level[level_end + 1..].trim().to_string();

                    return Some(LogEntry {
                        timestamp: (!timestamp.is_empty()).then(|| timestamp.to_string()),
                        level,
                        message,
                    });
                }
            }
        }
    }

    Some(LogEntry {
        timestamp: None,
        level: LogLevel::Unknown("UNSTRUCTURED".to_string()),
        message: line.to_string(),
    })
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LogSummary {
    pub total_entries: usize,
    pub by_level: BTreeMap<LogLevel, usize>,
}

pub fn summarize<'a>(entries: impl IntoIterator<Item = &'a LogEntry>) -> LogSummary {
    let mut summary = LogSummary::default();

    for entry in entries {
        summary.total_entries += 1;
        *summary.by_level.entry(entry.level.clone()).or_insert(0) += 1;
    }

    summary
}

pub fn filter_by_level<'a>(entries: &'a [LogEntry], level: &LogLevel) -> Vec<&'a LogEntry> {
    entries.iter().filter(|entry| &entry.level == level).collect()
}

pub fn parse_contents(contents: &str) -> Vec<LogEntry> {
    contents.lines().filter_map(parse_line).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_structured_log_line() {
        let entry =
            parse_line("[2026-09-18T08:42:11Z] [ERROR] Database connection failed").unwrap();

        assert_eq!(entry.timestamp.as_deref(), Some("2026-09-18T08:42:11Z"));
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Database connection failed");
    }

    #[test]
    fn preserves_unstructured_lines() {
        let entry = parse_line("legacy service started").unwrap();

        assert_eq!(entry.timestamp, None);
        assert_eq!(
            entry.level,
            LogLevel::Unknown("UNSTRUCTURED".to_string())
        );
        assert_eq!(entry.message, "legacy service started");
    }

    #[test]
    fn summarizes_entries_by_level() {
        let entries = parse_contents(
            "[2026-09-18T08:00:00Z] [INFO] Started\n\
             [2026-09-18T08:00:01Z] [WARN] Slow response\n\
             [2026-09-18T08:00:02Z] [ERROR] Request failed\n\
             [2026-09-18T08:00:03Z] [ERROR] Retry failed",
        );

        let summary = summarize(&entries);

        assert_eq!(summary.total_entries, 4);
        assert_eq!(summary.by_level.get(&LogLevel::Info), Some(&1));
        assert_eq!(summary.by_level.get(&LogLevel::Warn), Some(&1));
        assert_eq!(summary.by_level.get(&LogLevel::Error), Some(&2));
    }
}
