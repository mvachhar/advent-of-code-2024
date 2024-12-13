use clap::Parser;
use day8::board::{self};

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let board = board::read_board(&args.file, |x| x).unwrap();
    println!("{}x{}", board.ncols(), board.nrows());
}
