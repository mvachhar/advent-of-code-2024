use clap::Parser;
use day8::read_board;
use day8::count_antinodes;

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut map = read_board(&args.file).unwrap();
    println!("{}x{}", map.ncols(), map.nrows());
    let num_antinodes = count_antinodes(&mut map).unwrap();
    println!("num antinodes: {}", num_antinodes);
}
