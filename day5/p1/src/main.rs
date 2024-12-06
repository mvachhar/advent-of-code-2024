use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct InputParseError {
    message: String,
}

impl std::fmt::Display for InputParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InputParseError {}

impl From<String> for InputParseError {
    fn from(message: String) -> Self {
        InputParseError { message }
    }
}

fn parse_input(file: String) -> Result<(HashMap<u32, Vec<u32>>, Vec<Vec<u32>>), Box<dyn Error>> {
    let file = File::open(file)?;
    let reader = BufReader::new(file);
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates = Vec::new();

    let mut line_no = 0;
    let mut lines_iter = reader.lines().enumerate();
    // Parse the rules
    for (i, line_raw) in lines_iter.by_ref() {
        line_no = i + 1;
        let line =
            line_raw.map_err(|e| format!("I/O Error reading rules, line {}: {}", line_no, e))?;

        if line.is_empty() {
            break;
        }
        let (p1s, p2s) = match line.split_once("|") {
            Some((p1, p2)) => (p1, p2),
            None => {
                return Err(Box::new(InputParseError {
                    message: format!("Line {}: Expected '|' separator", line_no),
                }))
            }
        };
        let p1 = p1s
            .parse::<u32>()
            .map_err(|e| format!("Line {}: Expected number, got {}: {}", line_no, p1s, e))?;
        let p2 = p2s
            .parse::<u32>()
            .map_err(|e| format!("Line {}: Expected number, got {}: {}", line_no, p2s, e))?;

        let rule = rules.entry(p1).or_insert(vec![]);
        rule.push(p2);
    }

    let rules_last_line = line_no;
    // Parse the updates
    for (i, line_raw) in lines_iter {
        let line_no = rules_last_line + i + 1;
        let line =
            line_raw.map_err(|e| format!("I/O Error reading updates, line {}: {}", line_no, e))?;

        let pages = line
            .split(',')
            .map(|page_str| {
                page_str.parse::<u32>().map_err(|e| {
                    format!("Line {}: Expected number, got {}: {}", line_no, page_str, e)
                })
            })
            .collect::<Result<Vec<u32>, String>>()?;

        updates.push(pages);
    }

    return Ok((rules, updates));
}

fn find_valid_updates<'a>(rules: &HashMap<u32, Vec<u32>>, updates: &'a Vec<Vec<u32>>) -> Vec<&'a Vec<u32>> {
    let mut valid_updates = Vec::new();
    // This is the dumb O(n^2*m) solution where n is the number of pages in the udpate
    // and m is the maximum number of rules for any page
    for update in updates {
        let mut valid = true;
        for (pos, page) in update.iter().enumerate() {
            let rule = rules.get(page);
            if rule.is_none() { continue; }
            // Is there a better way than unwrapping here?
            // the loop control flow seems to cause the helpers to be useless
            for rule_page in rule.unwrap() {
                for upage in &update[..pos] {
                    if upage == rule_page {
                        valid = false;
                        break;
                    }
                }
                if !valid { break; }
            }
            if !valid { break; }
        }
        if valid {
            valid_updates.push(update);
        }
    }
    return valid_updates;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_valid_updates_no_rules() {
        let rules = HashMap::new();
        let updates = vec![vec![1, 2, 3], vec![2, 1, 3]];
        let valid_updates = find_valid_updates(&rules, &updates);
        assert_eq!(valid_updates.len(), 2);
    }

    #[test]
    fn test_find_valid_updates_1_rule() {
        let mut rules = HashMap::new();
        rules.insert(1, vec![2]);
        let updates = vec![vec![1, 2, 3], vec![2, 1, 3]];
        let valid_updates = find_valid_updates(&rules, &updates);
        assert_eq!(valid_updates.len(), 1);
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let (rules, updates) = parse_input(args.file).unwrap();
    let updates_to_print = find_valid_updates(&rules, &updates);
    println!("valid updates: {}/{}", updates_to_print.len(), updates.len());
    let sum = updates_to_print.iter().map(|update| update[update.len()/2]).sum::<u32>();
    println!("sum: {}", sum);
}
