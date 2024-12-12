use clap::Parser;
use std::convert::TryFrom;

use day6::board::{self, Board, BoardIndex};
use day6::board::{DIR_SYM_DOWN, DIR_SYM_LEFT, DIR_SYM_RIGHT, DIR_SYM_UP, OBSTACLE_SYM};
use day6::dir_vec::{DirVec, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};

const OCCUPIED_SYM: u8 = b'X';

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

fn compute_positions(board: &mut Board) -> Result<u32, SimError> {
    let mut positions = 0;
    let (board_index, dir_sym_ref) = board::find(&board, |x| match *x {
        DIR_SYM_UP | DIR_SYM_DOWN | DIR_SYM_LEFT | DIR_SYM_RIGHT => true,
        _ => false,
    })
    .map_err(|e| SimError {
        message: format!("{}", e),
    })?;
    let dir_sym = *dir_sym_ref;
    let mut pos = DirVec::try_from(&board_index).map_err(|e| SimError {
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

        let cur_index = match BoardIndex::from_pos(&board, &pos) {
            Ok(idx) => Ok(idx),
            Err(e) => Err(SimError {
                message: format!("Illegal current position {}: {}", pos, e),
            }),
        }?;
        let cur_cell = board[cur_index.raw()];
        if cur_cell != OCCUPIED_SYM {
            positions += 1;
            board[cur_index.raw()] = OCCUPIED_SYM;
        }

        let new_pos = pos.add(&dir_vec);
        let new_index = match BoardIndex::from_pos(&board, &new_pos) {
            Ok(idx) => idx,
            Err(_) => return Ok(positions),
        };
        let new_cell = board[new_index.raw()];
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
        let mut board = Board::from_shape_vec(
            (3, 3),
            vec![b'^', b'.', b'.', b'.', b'.', b'.', b'.', b'.', b'.'],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 1);
    }

    #[test]
    fn test_compute_positions_1_turn() {
        let mut board = Board::from_shape_vec(
            (3, 3),
            vec![b'#', b'.', b'.', b'^', b'.', b'.', b'.', b'.', b'.'],
        )
        .unwrap();
        let positions = compute_positions(&mut board).unwrap();
        assert_eq!(positions, 3);
    }

    #[test]
    fn test_compute_positions_2_turns() {
        let mut board = Board::from_shape_vec(
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
        let mut board = Board::from_shape_vec(
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
    let mut board = board::read_board(&args.file, |x| x).unwrap();
    println!("{}x{}", board.ncols(), board.nrows());
    let positions = compute_positions(&mut board).unwrap();
    println!("num positions: {}", positions);
}
