use clap::Parser;
use ndarray::Array2;
use std::convert::TryFrom;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
struct DirVec(isize, isize);

impl DirVec {
    fn add(&self, other: &DirVec) -> DirVec {
        DirVec(self.0 + other.0, self.1 + other.1)
    }
}

impl Display for DirVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}, {}>", self.0, self.1)
    }
}

const DIR_LEFT: DirVec = DirVec(-1, 0);
const DIR_RIGHT: DirVec = DirVec(1, 0);
const DIR_UP: DirVec = DirVec(0, -1);
const DIR_DOWN: DirVec = DirVec(0, 1);

const DIR_SYM_LEFT: u8 = b'<';
const DIR_SYM_RIGHT: u8 = b'>';
const DIR_SYM_UP: u8 = b'^';
const DIR_SYM_DOWN: u8 = b'v';

const OBSTACLE_SYM: u8 = b'#';
const OCCUPIED_SYM: u8 = b'X';

fn read_map(path: String) -> io::Result<Array2<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let mut cols = None;

    let mut buffer = Vec::new();
    while reader.read_until(b'\n', &mut buffer)? > 0 {
        let line_no = lines.len() + 1;
        let last_char_is_newline = buffer.ends_with(&[b'\n']);
        if last_char_is_newline {
            buffer.pop();
        }
        if cols.is_none() {
            cols = Some(buffer.len());
        } else {
            if buffer.len() != cols.unwrap() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid number of columns ({}) on line {}, expected {}",
                        buffer.len(),
                        line_no,
                        cols.unwrap()
                    ),
                ));
            }
        }

        lines.push(buffer);
        buffer = Vec::new();
    }
    if lines.len() == 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "No lines found in file",
        ));
    }

    let board = Array2::from_shape_vec(
        (lines.len(), cols.unwrap()),
        lines.into_iter().flatten().collect(),
    )
    .unwrap();

    Ok(board)
}

use std::fmt;

#[derive(Debug)]
struct SimError {
    message: String,
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SimError {}

fn find_guard(board: &Array2<u8>) -> Result<([usize; 2], u8), SimError> {
    for row in 0..board.nrows() {
        for col in 0..board.ncols() {
            let cell = board[[row, col]];
            let is_guard = match cell {
                DIR_SYM_UP | DIR_SYM_DOWN | DIR_SYM_LEFT | DIR_SYM_RIGHT => true,
                _ => false,
            };
            if is_guard {
                return Ok(([row, col], cell));
            }
        }
    }
    return Err(SimError {
        message: String::from("No guard found"),
    });
}

#[derive(Debug)]
struct OutOfBoundsError {
    message: String,
}

impl OutOfBoundsError {
    fn new<T: std::fmt::Display + PartialOrd>(
        val_desc: &str,
        val: T,
        bound: T,
    ) -> OutOfBoundsError {
        if val > bound {
            return OutOfBoundsError {
                message: format!("{} is out of bounds: {} > {}", val_desc, val, bound),
            };
        }
        if val < bound {
            return OutOfBoundsError {
                message: format!("{} is out of bounds: {} < {}", val_desc, val, bound),
            };
        }
        panic!("Invalid bounds error");
    }
}

impl fmt::Display for OutOfBoundsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OutOfBoundsError {}

fn pos_to_index(board: &Array2<u8>, pos: &DirVec) -> Result<[usize; 2], OutOfBoundsError> {
    let col = usize::try_from(pos.0).map_err(|_| OutOfBoundsError::new("x position", pos.0, 0))?;
    let row = usize::try_from(pos.1).map_err(|_| OutOfBoundsError::new("y position", pos.1, 0))?;
    if row >= board.nrows() {
        return Err(OutOfBoundsError::new("y position", row, board.nrows() - 1));
    }
    if col >= board.ncols() {
        return Err(OutOfBoundsError::new("x position", col, board.ncols() - 1));
    }
    return Ok([row, col]);
}

fn index_to_pos(index: [usize; 2]) -> Result<DirVec, <isize as TryFrom<usize>>::Error> {
    let x = isize::try_from(index[1])?;
    let y = isize::try_from(index[0])?;
    return Ok(DirVec(x, y));
}

fn dir_sym_to_vec(dir_sym: u8) -> Result<DirVec, SimError> {
    match dir_sym {
        DIR_SYM_LEFT => Ok(DIR_LEFT),
        DIR_SYM_RIGHT => Ok(DIR_RIGHT),
        DIR_SYM_UP => Ok(DIR_UP),
        DIR_SYM_DOWN => Ok(DIR_DOWN),
        _ => Err(SimError {
            message: format!("Invalid direction symbol {}", dir_sym),
        }),
    }
}

fn compute_positions(board: &mut Array2<u8>) -> Result<u32, SimError> {
    let mut positions = 0;
    let (board_index, dir_sym) = find_guard(&board)?;
    let mut pos = index_to_pos(board_index).map_err(|e| SimError {
        message: format!("Invalid board index for guard: {}", e),
    })?;
    let mut dir_vec = dir_sym_to_vec(dir_sym)?;
    let mut iters = 0;
    const MAX_ITERS: u32 = 10000000;
    loop {
        iters += 1;
        if iters > MAX_ITERS {
            return Err(SimError {
                message: format!("Max iterations reached: {}", MAX_ITERS),
            });
        }

        let cur_index = match pos_to_index(&board, &pos) {
            Ok(idx) => Ok(idx),
            Err(e) => Err(SimError { message: format!("Illegal current position {}: {}", pos, e) }),
        }?;
        let cur_cell = board[cur_index];
        if cur_cell != OCCUPIED_SYM {
            positions += 1;
            board[cur_index] = OCCUPIED_SYM;
        }

        let new_pos = pos.add(&dir_vec);
        let new_index = match pos_to_index(&board, &new_pos) {
            Ok(idx) => idx,
            Err(_) => return Ok(positions),
        };
        let new_cell = board[new_index];
        if new_cell == OBSTACLE_SYM {
            dir_vec = match dir_vec {
                DIR_LEFT => DIR_UP,
                DIR_RIGHT => DIR_DOWN,
                DIR_UP => DIR_RIGHT,
                DIR_DOWN => DIR_LEFT,
                _ => {
                    return Err(SimError {
                        message: format!("Invalid direction vector {}", dir_sym),
                    })
                }
            };
        } else {
            pos = new_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_positions_trivial() {
        let mut board = Array2::from_shape_vec(
            (3, 3),
            vec![b'^', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 1);
    }

    #[test]
    fn test_compute_positions_1_turn() {
        let mut board = Array2::from_shape_vec(
            (3, 3),
            vec![b'#', b'.', b'.', b'^', b'.', b'.', b'.', b'.', b'.'],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 3);
    }

    #[test]
    fn test_compute_positions_2_turns() {
        let mut board = Array2::from_shape_vec(
            (4, 3),
            vec![
                b'#', b'.', b'.', b'^', b'.', b'#', b'.', b'.', b'.', b'.', b'.', b'.',
            ],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 4);
    }

    #[test]
    fn test_compute_positions_overlap() {
        let mut board = Array2::from_shape_vec(
            (4, 3),
            vec![
                b'#', b'.', b'.', b'^', b'.', b'#', b'.', b'#', b'.', b'.', b'.', b'.',
            ],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 2);
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut board = read_map(args.file).unwrap();
    println!("{}x{}", board.ncols(), board.nrows());
    let positions = compute_positions(&mut board).unwrap();
    println!("num positions: {}", positions);
}
