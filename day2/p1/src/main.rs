use clap::Parser;
use std::fs;
use std::cmp::Ordering::{Less, Greater};


#[derive(Parser, Debug)]
struct Args {
    file: String,
}

fn parse_reports(file_path: &str) -> Result<Vec<Vec<i32>>, String> {
    let contents = fs::read_to_string(file_path).map_err(|_| "Could not read file".to_string())?;
    let mut reports = Vec::new();

    for line in contents.lines() {
        let levels_raw: Vec<&str> = line.split_whitespace().collect();
        let levels: Vec<i32> = levels_raw
            .iter()
            .map(|s| s.parse().map_err(|_| format!("Failed to parse '{}'", s)))
            .collect::<Result<Vec<i32>, String>>()?;
        reports.push(levels);
    }

    Ok(reports)
}

fn check_report(report: &Vec<i32>) -> i32 {
    if report.len() <= 1 {
        return 1;
    }
    
    let l0 = report.get(0).unwrap();
    let l1 = report.get(1).unwrap();
    let mut ord: std::cmp::Ordering = Greater;

    if l0 < l1 {
        ord = Less;
    }
    
    let mut prev_level = l0;
    for level in &report[1..] {
        if prev_level.cmp(level) != ord {
            return 0;
        }
        if (prev_level - level).abs() > 3 {
            return 0;
        }
        prev_level = level;
    }
    return 1;
}

fn main() {
    let args = Args::parse();
    let reports = parse_reports(&args.file).unwrap();
    let mut safe = 0;
    for report in reports {
        safe += check_report(&report);
    }
    println!("safe: {}", safe);
}
