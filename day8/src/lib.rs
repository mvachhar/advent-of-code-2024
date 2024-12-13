pub mod dir_vec;
pub mod board;

use std::error::Error;
use board::{Board, BoardIndex, BoardError};

#[derive(Debug, Clone)]
pub struct MapLoc {
    pub sym: Option<u8>,
    pub has_antinode: bool,
}

pub type Map = Board<MapLoc>;
pub type MapIndex = BoardIndex;

fn mark_antinode(map: &mut Map, antinode_index: &MapIndex) -> Result<usize, BoardError> {
    let antinode_loc = match map.get(antinode_index.raw()){
        Some(loc) => loc,
        None => {
            let (field, val, bound) = if antinode_index.row() >= map.nrows() {
                ("row", antinode_index.row(), map.nrows() - 1)
            } else if antinode_index.col() >= map.ncols() {
                ("col", antinode_index.col(), map.ncols() - 1)
            } else {
                ("unknown", 0, 0)
            };
            return Err(BoardError::new_oob(&format!("Antinode index {} out of bounds", field), val, bound));
        }
    };
    if !antinode_loc.has_antinode {
        map[antinode_index.raw()].has_antinode = true;
        return Ok(1);
    }
    return Ok(0);
}

fn mark_antinodes<F>(map: &mut Map, index1: &MapIndex, index2: &MapIndex, antinode_locs: &F) -> Result<usize, Box<dyn Error>>
where 
    F: Fn(&Map, &MapIndex, &MapIndex) -> Result<Vec<MapIndex>, Box<dyn Error>>
{
    let mut count = 0;
    let locs = antinode_locs(map, index1, index2)?;

    for loc in locs {
        count += mark_antinode(map, &loc)?;
    }
    Ok(count)
}

fn find_new_antinodes<F>(map: &mut Map, index: MapIndex, sym: u8, antinode_locs: &F) -> Result<usize, Box<dyn Error>>
where
    F: Fn(&Map, &MapIndex, &MapIndex) -> Result<Vec<MapIndex>, Box<dyn Error>>
{
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
                        count += mark_antinodes(map, &index, &new_index, antinode_locs)?;
                    }
                }
                None => {}
            }
        }
    }
    return Ok(count);
}

pub fn count_antinodes<F>(map: &mut Map, antinode_locs: &F) -> Result<usize, Box<dyn Error>> 
where
    F: Fn(&Map, &MapIndex, &MapIndex) -> Result<Vec<MapIndex>, Box<dyn Error>>
{
    let mut count = 0;
    for i in 0..map.nrows() {
        for j in 0..map.ncols() {
            // [i, j] cannot be out of bounds here
            let index = MapIndex::from_raw(map, &[i, j]).unwrap();
            let loc = map.get(index.raw()).unwrap();
            match loc.sym {
                Some(sym) => {
                    count += find_new_antinodes(map, index, sym, antinode_locs)?;
                }
                None => {}
            }
        }
    }
    Ok(count)
}

pub fn read_board(path: &str) -> Result<Map, Box<dyn Error>> {
    Ok(board::read_board(
        path,
        |x| MapLoc { 
            sym: if x != b'.' { Some(x) } else { None }, 
            has_antinode: false 
        }
    )?)
}
