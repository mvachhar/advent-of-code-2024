use std::fs;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct PossibleCalibrationEq {
    pub result: u64,
    pub args: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Add,
    Mul,
    Concat,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Op::Add => "+",
            Op::Mul => "*",
            Op::Concat => "||",
        })
    }
}

#[derive(Debug, Clone)]
pub struct CalibrationEq {
    pub result: u64,
    pub ops: Vec<Op>,
    pub args: Vec<u64>,
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

pub fn read_input(file: &str) -> Result<Vec<PossibleCalibrationEq>, io::Error> {
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
            Op::Concat => {
                let mut s = acc.to_string();
                s.push_str(&args[i+1].to_string());
                s.parse::<u64>().map_err(|_| ())?
            },
        }
    }
    return Ok(acc);
}

fn build_calibration_eq<'a>(result: u64, args: &[u64], legal_ops: &[Op], ops: &'a mut Vec<Op>) -> Result<(), ()> {
    if ops.len() == args.len() - 1 {
        let eresult = match eval(args, ops) {
            Ok(value) => value,
            Err(_) => return Err(()),
        };
        if eresult == result {
            return Ok(());
        } else {
            return Err(());
        }
    }
    
    for op in legal_ops {
        ops.push(*op);
        let res  = build_calibration_eq(result, args, legal_ops, ops);
        match res {
            Ok(_) => return Ok(()),
            Err(_) => (),
        }
        ops.pop();
    }
    return Err(());
}

pub fn build_valid_eq(eq: &PossibleCalibrationEq, legal_ops: &[Op]) -> Option<CalibrationEq> {
    let result = eq.result;
    let args = &eq.args;
    
    let mut ops = vec![];
    let res = build_calibration_eq(result, args, legal_ops, &mut ops);
    match res {
        Ok(_) => (),
        Err(_) => return None,
    };
    // Need to clone here since Calibration Eq owns the ops and args
    // Use unwrap here since this should never fail at this point
    return Some(CalibrationEq::new(result, args, &ops).unwrap());
}
