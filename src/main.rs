use rust_log_analyzer::{filter_by_level, parse_contents, summarize, LogLevel};
use std::env;
use std::fs;
use std::process;

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <log-file> [--level <TRACE|DEBUG|INFO|WARN|ERROR>]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage(&args[0]);
        process::exit(2);
    }

    let file_path = &args[1];
    let level_filter = match args.as_slice() {
        [_, _, flag, level] if flag == "--level" => Some(LogLevel::parse(level)),
        [_, _] => None,
        _ => {
            print_usage(&args[0]);
            process::exit(2);
        }
    };

    let contents = fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!("Failed to read '{file_path}': {error}");
        process::exit(1);
    });

    let entries = parse_contents(&contents);

    if let Some(level) = level_filter {
        let matches = filter_by_level(&entries, &level);
        println!("Entries matching {}: {}", level.as_str(), matches.len());

        for entry in matches {
            let timestamp = entry.timestamp.as_deref().unwrap_or("no-timestamp");
            println!(
                "[{timestamp}] [{}] {}",
                entry.level.as_str(),
                entry.message
            );
        }

        return;
    }

    let summary = summarize(&entries);
    println!("Log summary for: {file_path}");
    println!("Total entries: {}", summary.total_entries);

    for (level, count) in summary.by_level {
        println!("{:>12}: {count}", level.as_str());
    }
}
