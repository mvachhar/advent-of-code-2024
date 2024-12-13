use clap::Parser;
use day8::board::BoardErrorKind;
use day8::{count_antinodes, dir_vec::DirVec, read_board, MapIndex};

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut map = read_board(&args.file).unwrap();
    println!("{}x{}", map.ncols(), map.nrows());
    let num_antinodes = count_antinodes(&mut map, &|map, i1, i2| {
        let pos1 = DirVec::try_from(i1)?;
        let pos2 = DirVec::try_from(i2)?;
        let dirs = [pos2.sub(&pos1), pos1.sub(&pos2)];

        let mut ret = vec![];
        for dir in dirs {
            // We can do all computation from pos1 since pos2 == pos1 +/- dir
            let mut new_pos = pos1;
            loop {
                let new_index_res = MapIndex::from_pos(&map, &new_pos);
                match new_index_res {
                    Ok(new_index) => {
                        ret.push(new_index);
                    }
                    Err(e) => match e.kind {
                        BoardErrorKind::OutOfBounds => break,
                        _ => return Err(Box::new(e)),
                    },
                }
                // There is always an antinode at the antenna so update pos at the end here
                new_pos = new_pos.add(&dir);
            }
        }
        return Ok(ret);
    })
    .unwrap();
    println!("num antinodes: {}", num_antinodes);
}
