use clap::Parser;
use ndarray::Array2;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_puzzle_board(path: &str) -> io::Result<Array2<u8>> {
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

fn find_words(board: Array2<u8>, word: &[u8]) -> usize {
    let mut count = 0;
    let directions: [(isize, isize); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let ncols = isize::try_from(board.ncols()).unwrap();
    let nrows = isize::try_from(board.nrows()).unwrap();
    for x in 0..ncols {
        for y in 0..nrows {
            // x and y should always be positive, so this conversion should always be valid so the unwrap should be safe
            if board[[y.try_into().unwrap(), x.try_into().unwrap()]] != word[0] {
                continue;
            }
            for dir in directions {
                let mut x_pos = x;
                let mut y_pos = y;
                let mut found = true;
                for char in word.iter().skip(1) {
                    x_pos += dir.0;
                    y_pos += dir.1;
                    if x_pos < 0
                        || y_pos < 0
                        || x_pos >= ncols
                        || y_pos >= nrows
                        // By this point out of bounds and negative values should be filtered out
                        // so the unwrap should be safe
                        || board[[usize::try_from(y_pos).unwrap(), usize::try_from(x_pos).unwrap()]] != *char
                    {
                        found = false;
                        break;
                    }
                }
                if found {
                    count += 1;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_words() {
        let board = Array2::from_shape_vec(
            (4, 4),
            vec![b'X', b'M', b'A', b'S',
                b'M', b'M', b'.', b'.',
                b'A', b'.', b'A', b'.',
                b'S', b'X', b'.', b'S'],
        )
        .unwrap();
        let word = "XMAS".as_bytes();
        let count = find_words(board, word);
        assert_eq!(count, 3);
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let board = read_puzzle_board(&args.file).unwrap();
    let xmas_count = find_words(board, "XMAS".as_bytes());
    println!("xmas count {}", xmas_count);
}
