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

fn find_words(board: Array2<u8>) -> usize {
    let mut count = 0;
    let patterns = vec![
        [[b'M', b'0', b'M'], [b'0', b'A', b'0'], [b'S', b'0', b'S']],
        [[b'S', b'0', b'M'], [b'0', b'A', b'0'], [b'S', b'0', b'M']],
        [[b'M', b'0', b'S'], [b'0', b'A', b'0'], [b'M', b'0', b'S']],
        [[b'S', b'0', b'S'], [b'0', b'A', b'0'], [b'M', b'0', b'M']],
    ];

    let ncols = board.ncols();
    let nrows = board.nrows();
    for x in 0..ncols {
        for y in 0..nrows {
            // We could be more clever here and check for the A and then look 
            // diagonally for the other letters, but this is simpler.
            for pattern in &patterns {
                let mut found = true;
                if x + 3 > ncols || y + 3 > nrows {
                    continue;
                }
                for x_offset in 0..3 {
                    for y_offset in 0..3 {
                        let pc = pattern[y_offset][x_offset];
                        if pc == b'0' { continue; }

                        if !(board[[y + y_offset, x + x_offset]] == pc) {
                            found = false;
                            break;
                        }
                    }
                }
                if found {
                    count += 1;
                    break;
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
            vec![b'.', b'M', b'.', b'S',
                b'.', b'.', b'A', b'.',
                b'.', b'M', b'.', b'S',
                b'S', b'X', b'.', b'S'],
        )
        .unwrap();
        let count = find_words(board);
        assert_eq!(count, 1);
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let board = read_puzzle_board(&args.file).unwrap();
    let xmas_count = find_words(board);
    println!("xmas count {}", xmas_count);
}
