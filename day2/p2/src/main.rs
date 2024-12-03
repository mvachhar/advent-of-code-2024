use clap::Parser;
use std::cmp::Ordering::{Greater, Less};
use std::fs;

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

const MAX_DIFF: i32 = 3;

fn check_report_no_skip(report: &[i32]) -> i32 {
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
        if (prev_level - level).abs() > MAX_DIFF {
            return 0;
        }
        prev_level = level;
    }
    return 1;
}

// This is O(n^2) but it isn't clear to me how to keep it simple and O(n)
// Not worth the time to do complex and O(n)
fn check_report(report: &[i32]) -> i32{
    let pass = check_report_no_skip(report);
    if pass > 0{
        return pass;
    }
    for pos in 0..report.len() {
        let prefix = &report[..pos];
        let suffix = &report[pos+1..];
        let mut skipped = Vec::new();
        skipped.extend_from_slice(prefix);
        skipped.extend_from_slice(suffix);
        let pass = check_report_no_skip(&skipped);
        if pass > 0 {
            return pass;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_report_nonleading() {
        assert_eq!(check_report(&[1, 2, 3, 4]), 1); //Good
        assert_eq!(check_report(&[1, 3, 3, 4]), 1); // Eq element
        assert_eq!(check_report(&[1, 3, 3, 4, 4]), 0); // 2 Eq element
        assert_eq!(check_report(&[1, 5, 7, 9, 33]), 0); // 2 Gap too large
    }

    #[test]
    fn test_check_report_leading() {
        assert_eq!(check_report(&[2, 1, 2, 3, 4]), 1); // Skip first, ooo
        assert_eq!(check_report(&[7, 1, 2, 3, 4]), 1); // Skip first, ooo and gap too large
        assert_eq!(check_report(&[1, 10, 11, 12, 13]), 1); // Skip second, gap too large
        assert_eq!(check_report(&[16, 15, 19, 22, 23, 25]), 1); // 15->19 gap too large but 15 should be dropped, not 19
    }

    #[test]
    fn test_examples() {
        assert_eq!(check_report(&[7, 6, 4, 2, 1]), 1);
        assert_eq!(check_report(&[1, 2, 7, 8, 9]), 0);
        assert_eq!(check_report(&[9, 7, 6, 2, 1]), 0);
        assert_eq!(check_report(&[1, 3, 2, 4, 5]), 1);
        assert_eq!(check_report(&[8, 6, 4, 4, 1]), 1);
        assert_eq!(check_report(&[1, 3, 6, 7, 9]), 1);
    }
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
