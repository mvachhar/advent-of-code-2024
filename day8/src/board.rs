use crate::dir_vec::DirVec;
use ndarray::Array2;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Board<T> = Array2<T>;
#[derive(Debug)]
pub struct BoardIndex([usize; 2]);

#[derive(Debug)]
pub enum BoardErrorKind {
    OutOfBounds,
    NoMatch,
}

#[derive(Debug)]
pub struct BoardError {
    pub kind: BoardErrorKind,
    pub message: String,
}

pub const DIR_SYM_LEFT: u8 = b'<';
pub const DIR_SYM_RIGHT: u8 = b'>';
pub const DIR_SYM_UP: u8 = b'^';
pub const DIR_SYM_DOWN: u8 = b'v';

pub const OBSTACLE_SYM: u8 = b'#';
pub const EMPTY_SYM: u8 = b'.';

impl BoardError {
    pub fn new_oob<T: std::fmt::Display + PartialOrd>(
        val_desc: &str,
        val: T,
        bound: T,
    ) -> BoardError {
        if val > bound {
            return BoardError {
                kind: BoardErrorKind::OutOfBounds,
                message: format!("{} is out of bounds: {} > {}", val_desc, val, bound),
            };
        }
        if val < bound {
            return BoardError {
                kind: BoardErrorKind::OutOfBounds,
                message: format!("{} is out of bounds: {} < {}", val_desc, val, bound),
            };
        }
        panic!("Invalid bounds error");
    }
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BoardError {}

pub fn read_board<T, F>(path: &str, f: F) -> io::Result<Array2<T>>
where
    F: Fn(u8) -> T,
{
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
        lines.into_iter().flatten().map(f).collect(),
    )
    .unwrap();

    Ok(board)
}

pub fn find<T, F>(board: &Board<T>, f: F) -> Result<(BoardIndex, &T), BoardError>
where
    F: Fn(&T) -> bool,
{
    for row in 0..board.nrows() {
        for col in 0..board.ncols() {
            let cell = &board[[row, col]];
            if f(cell) {
                return Ok((BoardIndex([row, col]), cell));
            }
        }
    }
    return Err(BoardError {
        kind: BoardErrorKind::NoMatch,
        message: String::from("No match found"),
    });
}

impl BoardIndex {
    pub fn raw(&self) -> [usize; 2] {
        self.0
    }

    pub fn from_raw<T>(board: &Board<T>, ri: &[usize; 2]) -> Result<BoardIndex, BoardError> {
        let row = ri[0];
        let col = ri[1];
        if row >= board.nrows() {
            return Err(BoardError::new_oob("y position", row, board.nrows() - 1));
        }
        if col >= board.ncols() {
            return Err(BoardError::new_oob("x position", col, board.ncols() - 1));
        }
        return Ok(BoardIndex(*ri));
    }

    pub fn from_pos<T>(board: &Board<T>, pos: &DirVec) -> Result<BoardIndex, BoardError> {
        let col =
            usize::try_from(pos.0).map_err(|_| BoardError::new_oob("x position", pos.0, 0))?;
        let row =
            usize::try_from(pos.1).map_err(|_| BoardError::new_oob("y position", pos.1, 0))?;
        if row >= board.nrows() {
            return Err(BoardError::new_oob("y position", row, board.nrows() - 1));
        }
        if col >= board.ncols() {
            return Err(BoardError::new_oob("x position", col, board.ncols() - 1));
        }
        return Ok(BoardIndex([row, col]));
    }

    pub fn row(&self) -> usize {
        self.0[0]
    }

    pub fn col(&self) -> usize {
        self.0[1]
    }
}
