use clap::Parser;
use day7::{read_input, build_valid_eq, CalibrationEq, Op};

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let equations = read_input(&args.file).unwrap();
    let valid_eqs = equations
        .iter()
        .filter_map(|eq| build_valid_eq(eq, &[Op::Add, Op::Mul, Op::Concat]))
        .collect::<Vec<CalibrationEq>>();
    println!("Found {} valid equations", valid_eqs.len());
    let sum = valid_eqs.iter().map(|eq| eq.result).sum::<u64>();
    println!("Sum of results: {}", sum);
}
