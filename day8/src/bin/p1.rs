use std::error::Error;

use clap::Parser;
use day8::dir_vec::DirVec;
use day8::board::{self, Board, BoardIndex};

#[derive(Debug, Clone)]
struct MapLoc {
    sym: Option<u8>,
    has_antinode: bool,
}

type Map = Board<MapLoc>;
type MapIndex = BoardIndex;

#[derive(Parser)]
struct Args {
    file: String,
}

fn mark_antinode(map: &mut Map, pos: &DirVec) -> Result<usize, board::BoardError> {
    let antinode_index_res = MapIndex::from_pos(map, &pos);
    let antinode_index = match antinode_index_res {
        Ok(index) => index,
        Err(e) => {
            match e.kind {
                board::BoardErrorKind::OutOfBounds => return Ok(0),
                _ => return Err(e),
            }
        }
    };

    // At this point, antinode_index is valid
    let antinode_loc = map.get(antinode_index.raw()).unwrap();
    if !antinode_loc.has_antinode {
        map[antinode_index.raw()].has_antinode = true;
        return Ok(1);
    }
    return Ok(0);
}

fn mark_antinodes(map: &mut Map, index1: &MapIndex, index2: &MapIndex) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    let pos1 = DirVec::try_from(index1)?;
    let pos2 = DirVec::try_from(index2)?;
    let antinode_dir1 = pos2.sub(&pos1);
    let antinode_dir2 = antinode_dir1.neg();
    let antinode_pos1 = pos2.add(&antinode_dir1);
    let antinode_pos2 = pos1.add(&antinode_dir2);

    count += mark_antinode(map, &antinode_pos1)?;
    count += mark_antinode(map, &antinode_pos2)?;
    Ok(count)
}

fn find_new_antinodes(map: &mut Map, index: MapIndex, sym: u8) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    // Only search forward on the map since earlier 
    // antennas would already be handled
    for i in index.row()..map.nrows() {
        let col_start = if i == index.row() { index.col() + 1 } else { 0 };
        for j in col_start..map.ncols() {
            // [i, j] cannot be out of bounds here
            let new_index = MapIndex::from_raw(map, &[i, j]).unwrap();
            let loc = map.get(new_index.raw()).unwrap();
            match loc.sym {
                Some(new_sym) => {
                    if new_sym == sym {
                        count += mark_antinodes(map, &index, &new_index)?;
                    }
                }
                None => {}
            }
        }
    }
    return Ok(count);
}

fn count_antinodes(map: &mut Map) -> Result<usize, Box<dyn Error>> {
    let mut count = 0;
    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            // [i, j] cannot be out of bounds here
            let index = MapIndex::from_raw(map, &[i, j]).unwrap();
            let loc = map.get(index.raw()).unwrap();
            match loc.sym {
                Some(sym) => {
                    count += find_new_antinodes(map, index, sym)?;
                }
                None => {}
            }
        }
    }
    Ok(count)
}

fn main() {
    let args = Args::parse();
    let mut map = board::read_board(
        &args.file,
        |x| MapLoc { 
            sym: if x != b'.' { Some(x) } else { None }, 
            has_antinode: false 
        }
    ).unwrap();
    println!("{}x{}", map.ncols(), map.nrows());
    let num_antinodes = count_antinodes(&mut map).unwrap();
    println!("num antinodes: {}", num_antinodes);
}
