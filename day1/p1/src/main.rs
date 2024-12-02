use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    file: String,
}

fn parse_lists(file_path: &str) -> Result<(Vec<i32>, Vec<i32>), String> {
    let contents = fs::read_to_string(file_path)
        .map_err(|_| "Could not read file".to_string())?;
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    for (index, line) in contents.lines().enumerate() {
        let entries: Vec<&str> = line.split_whitespace().collect();
        if entries.len() != 2 {
            return Err(format!("Line {} does not contain exactly two entries", index + 1));
        }
        if let (Ok(entry1), Ok(entry2)) = (entries[0].parse::<i32>(), entries[1].parse::<i32>()) {
            array1.push(entry1);
            array2.push(entry2);
        }
    }

    Ok((array1, array2))
}

fn main() {
    let args = Args::parse();
    let (mut locations1, mut locations2) = parse_lists(&args.file).expect("Could not parse input lists");
    locations1.sort();
    locations2.sort();
    let mut distance = 0;
    for i in 0..locations1.len() {
        let difference = locations1[i] - locations2[i];
        distance += difference.abs();
    }
    println!("distance: {:?}", distance);
}
