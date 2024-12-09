use crate::dir_vec::DirVec;
use ndarray::Array2;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub type Board<T = u8> = Array2<T>;
pub struct BoardIndex([usize; 2]);

#[derive(Debug)]
pub struct OutOfBoundsError {
    message: String,
}

pub const DIR_SYM_LEFT: u8 = b'<';
pub const DIR_SYM_RIGHT: u8 = b'>';
pub const DIR_SYM_UP: u8 = b'^';
pub const DIR_SYM_DOWN: u8 = b'v';

pub const OBSTACLE_SYM: u8 = b'#';

impl OutOfBoundsError {
    pub fn new<T: std::fmt::Display + PartialOrd>(
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


impl BoardIndex {
    pub fn raw(&self) -> [usize; 2] {
        self.0
    }

    pub fn from_raw(board: &Board, ri: &[usize; 2]) -> Result<BoardIndex, OutOfBoundsError> {
        let row = ri[0];
        let col = ri[1];
        if row >= board.nrows() {
            return Err(OutOfBoundsError::new("y position", row, board.nrows() - 1));
        }
        if col >= board.ncols() {
            return Err(OutOfBoundsError::new("x position", col, board.ncols() - 1));
        }
        return Ok(BoardIndex(*ri));
    }

    pub fn from_pos(board: &Board, pos: &DirVec) -> Result<BoardIndex, OutOfBoundsError> {
        let col =
            usize::try_from(pos.0).map_err(|_| OutOfBoundsError::new("x position", pos.0, 0))?;
        let row =
            usize::try_from(pos.1).map_err(|_| OutOfBoundsError::new("y position", pos.1, 0))?;
            if row >= board.nrows() {
                return Err(OutOfBoundsError::new("y position", row, board.nrows() - 1));
            }
            if col >= board.ncols() {
                return Err(OutOfBoundsError::new("x position", col, board.ncols() - 1));
            }
        return Ok(BoardIndex([row, col]));
    }
}
