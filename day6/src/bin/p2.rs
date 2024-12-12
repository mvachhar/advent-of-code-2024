use clap::Parser;
use std::convert::TryFrom;
use std::io::{stdout, Write};

use day6::board::{self, BoardIndex, EMPTY_SYM};
use day6::board::{DIR_SYM_DOWN, DIR_SYM_LEFT, DIR_SYM_RIGHT, DIR_SYM_UP, OBSTACLE_SYM};
use day6::dir_vec::{DirVec, DIR_DOWN, DIR_LEFT, DIR_RIGHT, DIR_UP};

use std::fmt;

type PriorOccupancy = u8;
const PRIOR_OCCUPANCY_LEFT: u8 = 0b0001;
const PRIOR_OCCUPANCY_RIGHT: u8 = 0b0010;
const PRIOR_OCCUPANCY_UP: u8 = 0b0100;
const PRIOR_OCCUPANCY_DOWN: u8 = 0b1000;
const PRIOR_OCCUPANCY_NEVER: u8 = 0b0000;

#[derive(Debug, Clone)]
enum BoardState {
    Empty(PriorOccupancy),
    Obstacle,
    GuardLeft,
    GuardRight,
    GuardUp,
    GuardDown,
}

type Board = board::Board<BoardState>;

#[derive(Debug, Clone, Copy)]
enum SimErrorKind {
    InvalidBoardState,
    InvalidDirectionVector,
    MaxIterationsReached,
    LoopDetected,
    InvalidBoardSymbol,
    NoGuardFound,
}

#[derive(Debug, Clone)]
struct SimError {
    kind: SimErrorKind,
    message: String,
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SimError {}

impl TryFrom<&BoardState> for DirVec {
    type Error = SimError;

    fn try_from(value: &BoardState) -> Result<Self, Self::Error> {
        match value {
            BoardState::GuardLeft => Ok(DIR_LEFT),
            BoardState::GuardRight => Ok(DIR_RIGHT),
            BoardState::GuardUp => Ok(DIR_UP),
            BoardState::GuardDown => Ok(DIR_DOWN),
            _ => Err(SimError {
                kind: SimErrorKind::InvalidBoardSymbol,
                message: format!("Invalid direction symbol {:?}", value),
            }),
        }
    }
}

fn occupancy_from_dir_vec(dir_vec: &DirVec) -> Result<PriorOccupancy, SimError> {
    match *dir_vec {
        DIR_LEFT => Ok(PRIOR_OCCUPANCY_LEFT),
        DIR_RIGHT => Ok(PRIOR_OCCUPANCY_RIGHT),
        DIR_UP => Ok(PRIOR_OCCUPANCY_UP),
        DIR_DOWN => Ok(PRIOR_OCCUPANCY_DOWN),
        _ => Err(SimError {
            kind: SimErrorKind::InvalidDirectionVector,
            message: format!("Cannot convert direction vector {:?} to prior occupancy", dir_vec),
        }),
    }
}

fn compute_positions(board: &mut Board, board_index_start: &BoardIndex, dir_vec_start: &DirVec) -> Result<u32, SimError> {
    let mut positions = 0;
    let mut pos = DirVec::try_from(board_index_start).map_err(|e| SimError {
        kind: SimErrorKind::InvalidBoardState,
        message: format!("Invalid board index start: {}", e),
    })?;
    let mut dir_vec = dir_vec_start.clone(); 
    let mut iters = 0;
    const MAX_ITERS: u32 = 10000;
    loop {
        iters += 1;
        if iters > MAX_ITERS {
            return Err(SimError {
                kind: SimErrorKind::MaxIterationsReached,
                message: format!("Max iterations reached: {}", MAX_ITERS),
            });
        }

        let cur_index = match BoardIndex::from_pos(&board, &pos) {
            Ok(idx) => Ok(idx),
            Err(e) => Err(SimError {
                kind: SimErrorKind::InvalidBoardState,
                message: format!("Illegal current position {}: {}", pos, e),
            }),
        }?;
        let cur_cell = &board[cur_index.raw()];
        let new_occ = occupancy_from_dir_vec(&dir_vec)?;
        match cur_cell {
            BoardState::GuardLeft
            | BoardState::GuardRight
            | BoardState::GuardUp
            | BoardState::GuardDown => {
                positions += 1;
                board[cur_index.raw()] = BoardState::Empty(new_occ);
            }
            | BoardState::Empty(occ) => {
                if *occ == PRIOR_OCCUPANCY_NEVER {
                    positions += 1;
                }
                if (occ & new_occ) != 0 {
                    return Err(SimError {
                        kind: SimErrorKind::LoopDetected,
                        message: format!("Loop detected at position {:?}", pos),
                    });
                }
                board[cur_index.raw()] = BoardState::Empty(occ | new_occ);
            }
            BoardState::Obstacle => return Err(SimError {
                kind: SimErrorKind::InvalidBoardState,
                message: format!("Currently on top of obstacle at {:?}", pos),
            }),
        }
        
        let new_pos = pos.add(&dir_vec);
        let new_index = match BoardIndex::from_pos(&board, &new_pos) {
            Ok(idx) => idx,
            Err(_) => return Ok(positions),
        };
        let new_cell = &board[new_index.raw()];
        match new_cell {
            BoardState::Obstacle => {
                dir_vec = match dir_vec {
                    DIR_LEFT => DIR_UP,
                    DIR_RIGHT => DIR_DOWN,
                    DIR_UP => DIR_RIGHT,
                    DIR_DOWN => DIR_LEFT,
                    _ => {
                        return Err(SimError {
                            kind: SimErrorKind::InvalidDirectionVector,
                            message: format!("Invalid direction vector {:?} at position {:?}", dir_vec, pos),
                        })
                    }
                };
            }
            _ => {
                pos = new_pos;
            }
        }
    }
}

fn find_initial_guard_position(board: &mut Board) -> Result<(BoardIndex, DirVec), SimError> {
    let (board_index, board_state) = board::find(&board, |x| match *x {
        BoardState::GuardLeft
        | BoardState::GuardRight
        | BoardState::GuardUp
        | BoardState::GuardDown => true,
        _ => false,
    })
    .map_err(|e| SimError {
        kind: SimErrorKind::NoGuardFound,
        message: format!("{}", e),
    })?;
    let dir_vec = DirVec::try_from(board_state)?;
    return Ok((board_index, dir_vec));
}

fn reset_board_empty_positions(board: &mut Board) {
    for i in 0..board.nrows() {
        for j in 0..board.ncols() {
            let cell = &board[[i, j]];
            match cell {
                BoardState::Empty(_) => board[[i, j]] = BoardState::Empty(PRIOR_OCCUPANCY_NEVER),
                _ => (),
            }
        }
    }
}

fn find_num_loop_obstacle_positions(board: &mut Board) -> Result<u32, SimError> {
    let (guard_start_index, guard_initial_dir_vec) = find_initial_guard_position(board)?;
    board[guard_start_index.raw()] = BoardState::Empty(PRIOR_OCCUPANCY_NEVER);
    
    // This is the dumb O(n^2) way to do this, where n is the size of the board.
    let mut num_obstacle_positions = 0;
    for i in 0..board.nrows() {
        for j in 0..board.ncols() {
            if i == guard_start_index.row() && j == guard_start_index.col() {
                // Cannot place an obstacle at the starting position of the guard.
                continue;
            }
            let orig_cell = board[[i, j]].clone();
            match orig_cell {
                BoardState::Obstacle => continue,
                BoardState::GuardDown | 
                BoardState::GuardUp |
                BoardState::GuardLeft |
                BoardState::GuardRight => continue,
                BoardState::Empty(_) => {
                    board[[i, j]] = BoardState::Obstacle;
                }
            }
            let result = compute_positions(board, &guard_start_index, &guard_initial_dir_vec);
            let _moves = (match result {
                Ok(x) => Ok(Some(x)),
                Err(e) => {
                    match e.kind {
                        SimErrorKind::LoopDetected => { num_obstacle_positions += 1; Ok(None) },
                        _ => Err(e),
                    }
                }
            }).map_err(|e| SimError {
                kind: e.kind,
                message: format!("Error computing board positions for obstacle at {:?}: {}", [i, j], e),
            })?;
            board[[i, j]] = orig_cell;
            reset_board_empty_positions(board);
        }
        print!(".");
        stdout().flush().unwrap();
    }
    println!("");
    return Ok(num_obstacle_positions);
}

fn board_char_to_state(c: u8) -> Result<BoardState, ()> {
    match c {
        DIR_SYM_UP => Ok(BoardState::GuardUp),
        DIR_SYM_DOWN => Ok(BoardState::GuardDown),
        DIR_SYM_LEFT => Ok(BoardState::GuardLeft),
        DIR_SYM_RIGHT => Ok(BoardState::GuardRight),
        OBSTACLE_SYM => Ok(BoardState::Obstacle),
        EMPTY_SYM => Ok(BoardState::Empty(PRIOR_OCCUPANCY_NEVER)),
        _ => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board_chars_to_board_state<'a, T>(
        chars: T,
    ) -> impl Iterator<Item = Result<BoardState, ()>> + 'a
    where
        T: Iterator<Item = &'a u8> + 'a,
    {
        return chars.map(|c| board_char_to_state(*c));
    }

    fn board_chars_to_board_state_vec(chars: &Vec<Vec<u8>>) -> Vec<BoardState> {
        board_chars_to_board_state(chars.iter().flatten())
            .collect::<Result<Vec<BoardState>, ()>>()
            .unwrap()
    }

    #[test]
    fn test_compute_positions_trivial() {
        let mut board = Board::from_shape_vec(
            (3, 3),
            board_chars_to_board_state_vec(&vec![
                vec![b'^', b'.', b'.'],
                vec![b'.', b'.', b'.'],
                vec![b'.', b'.', b'.'],
            ]),
        )
        .unwrap();
        let (board_index, dir_vec) = find_initial_guard_position(&mut board).unwrap();
        let positions = compute_positions(&mut board, &board_index, &dir_vec).unwrap();
        assert_eq!(positions, 1);
    }

    #[test]
    fn test_compute_positions_1_turn() {
        let mut board = Board::from_shape_vec(
            (3, 3),
            board_chars_to_board_state_vec(&vec![
                vec![b'#', b'.', b'.'],
                vec![b'^', b'.', b'.'],
                vec![b'.', b'.', b'.'],
            ]),
        )
        .unwrap();
        let (board_index, dir_vec) = find_initial_guard_position(&mut board).unwrap();
        let positions = compute_positions(&mut board, &board_index, &dir_vec).unwrap();
        assert_eq!(positions, 3);
    }

    #[test]
    fn test_compute_positions_2_turns() {
        let mut board = Board::from_shape_vec(
            (4, 3),
            board_chars_to_board_state_vec(&vec![
                vec![b'#', b'.', b'.'],
                vec![b'^', b'.', b'#'],
                vec![b'.', b'.', b'.'],
                vec![b'.', b'.', b'.'],
            ]),
        )
        .unwrap();
        let (board_index, dir_vec) = find_initial_guard_position(&mut board).unwrap();
        let positions = compute_positions(&mut board, &board_index, &dir_vec).unwrap();
        assert_eq!(positions, 4);
    }

    #[test]
    fn test_compute_positions_overlap() {
        let mut board = Board::from_shape_vec(
            (4, 3),
            board_chars_to_board_state_vec(&vec![
                vec![b'#', b'.', b'.'],
                vec![b'^', b'.', b'#'],
                vec![b'.', b'#', b'.'],
                vec![b'.', b'.', b'.'],
            ]),
        )
        .unwrap();
        let (board_index, dir_vec) = find_initial_guard_position(&mut board).unwrap();
        let positions = compute_positions(&mut board, &board_index, &dir_vec).unwrap();
        assert_eq!(positions, 2);
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut board = board::read_board(&args.file, |c| board_char_to_state(c).unwrap()).unwrap();
    println!("{}x{}", board.ncols(), board.nrows());
    let positions = find_num_loop_obstacle_positions(&mut board).unwrap();
    println!("num positions: {}", positions);
}
