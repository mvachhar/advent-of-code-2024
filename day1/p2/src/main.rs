use clap::Parser;
use std::fs;
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}

fn parse_lists(file_path: &str) -> Result<(Vec<i32>, HashMap<i32, i32>), String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|_| "Could not read file".to_string())?;
    let mut locations1 = Vec::new();
    let mut locations2_counts = HashMap::new();

    for (index, line) in contents.lines().enumerate() {
        let entries: Vec<&str> = line.split_whitespace().collect();
        if entries.len() != 2 {
            return Err(format!("Line {} does not contain exactly two entries", index + 1));
        }
        if let (Ok(entry1), Ok(entry2)) = (entries[0].parse::<i32>(), entries[1].parse::<i32>()) {
            locations1.push(entry1);
            let cur = locations2_counts.get(&entry2).unwrap_or(&0);
            locations2_counts.insert(entry2, cur + 1);
        }
    }

    Ok((locations1, locations2_counts))
}

fn main() {
    let args = Args::parse();
    let (locations1, locations2_counts) = parse_lists(&args.file).expect("Could not parse input lists");
    let mut similarity = 0;
    for l in locations1.iter() {
        similarity += l * locations2_counts.get(l).unwrap_or(&0);
    }
    println!("similarty: {:?}", similarity);
}
