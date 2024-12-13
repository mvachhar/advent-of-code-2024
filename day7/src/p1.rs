use std::fs;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::fmt::Display;

use clap::Parser;

#[derive(Parser)]
struct Args {
    file: String,
}

#[derive(Debug, Clone)]
struct PossibleCalibrationEq {
    result: u64,
    args: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Op {
    Add,
    Mul,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Op::Add => "+",
            Op::Mul => "*",
        })
    }
}

#[derive(Debug, Clone)]
struct CalibrationEq {
    result: u64,
    ops: Vec<Op>,
    args: Vec<u64>,
}

impl CalibrationEq {
    pub fn new(result: u64, args: &Vec<u64>, ops: &Vec<Op>) -> Result<Self, ()> {
        if args.len() != ops.len() + 1 { return Err(()); }
        Ok(Self { result, args: args.clone(), ops: ops.clone() })
    }
}

impl Display for CalibrationEq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.result, self.args[0])?;
        for (i, op) in self.ops.iter().enumerate() {
            write!(f, "{}{}", op, self.args[i+1])?
        }
        Ok(())
    }
}

fn read_input(file: &str) -> Result<Vec<PossibleCalibrationEq>, io::Error> {
    let file = fs::File::open(file).expect("Something went wrong opening the file");
    let reader = std::io::BufReader::new(file);
    let mut ret = vec![];
    for (lineno, line) in reader.lines().enumerate() {
        let line = line?;
        let mut split = line.splitn(2, ":");
        let result = match split.next() {
            Some(value) => value,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not parse line {}, no result found", lineno),
                ))
            }
        }
        .parse::<u64>()
        .map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Could not parse result on line {}, {}", lineno, e),
            )
        })?;
        let rest = (match split.next() {
            Some(value) => value,
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not parse line {}, no values found", lineno),
                ))
            }
        })
        .trim();
        let args = rest
            .split(" ")
            .map(|s| s.trim().parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()
            .map_err(|e| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Could not parse args on line {}, {}", lineno, e),
                )
            })?;
        ret.push(PossibleCalibrationEq { result, args });
    }
    return Ok(ret);
}

fn eval(args: &[u64], ops: &[Op]) -> Result<u64, ()> {
    if ops.len() != args.len() - 1 {
        return Err(());
    }
    let mut acc = args[0];
    for (i, op) in ops.iter().enumerate() {
        acc = match op {
            Op::Add => acc + args[i+1],
            Op::Mul => acc * args[i+1],
        }
    }
    return Ok(acc);
}

fn build_calibration_eq<'a>(result: u64, args: &[u64], ops: &'a mut Vec<Op>) -> (Result<(), ()>, &'a mut Vec<Op>) {
    if ops.len() == args.len() - 1 {
        let eresult = match eval(args, ops) {
            Ok(value) => value,
            Err(_) => return (Err(()), ops),
        };
        if eresult == result {
            return (Ok(()), ops);
        } else {
            return (Err(()), ops);
        }
    }
    // This threading of ops is ugly, there must be a better way
    let mut ret_ops = ops;
    for op in [Op::Add, Op::Mul] {
        ret_ops.push(op);
        let (res, new_ops)  = build_calibration_eq(result, args, ret_ops);
        match res {
            Ok(_) => return (Ok(()), new_ops),
            Err(_) => (),
        }
        new_ops.pop();
        ret_ops = new_ops;
    }
    return (Err(()), ret_ops);
}

fn build_valid_eq(eq: &PossibleCalibrationEq) -> Option<CalibrationEq> {
    let result = eq.result;
    let args = &eq.args;
    
    let mut ops = vec![];
    let (res, ops) = build_calibration_eq(result, args, &mut ops);
    match res {
        Ok(_) => (),
        Err(_) => return None,
    };
    // Need to clone here since Calibration Eq owns the ops and args
    // Use unwrap here since this should never fail at this point
    return Some(CalibrationEq::new(result, args, ops).unwrap());
}

fn main() {
    let args = Args::parse();
    let equations = read_input(&args.file).unwrap();
    let valid_eqs = equations
        .iter()
        .filter_map(|eq| build_valid_eq(eq))
        .collect::<Vec<CalibrationEq>>();
    println!("Found {} valid equations", valid_eqs.len());
    let sum = valid_eqs.iter().map(|eq| eq.result).sum::<u64>();
    println!("Sum of results: {}", sum);
}
