use std::error::Error;

use day9::{DiskBlock, read_disk_map};

use clap::Parser;

// Sucks that this bloats to 64 bits, but not worth custom struct

fn find_free_block(disk_map: &Vec<DiskBlock>, last_free: usize) -> Option<usize> {
    for i in last_free..disk_map.len() {
        match disk_map[i] {
            DiskBlock::Free => return Some(i),
            _ => {}
        }
    }
    return None;
}

fn compact_disk_map(disk_map: &mut Vec<DiskBlock>) -> () {
    let mut last_free = 0;
    let mut i = disk_map.len();
    while i > 0 {
        i -= 1;
        let block = &disk_map[i];
        match block {
            DiskBlock::Free => continue,
            _ => {}
        }
        let next_free = match find_free_block(disk_map, last_free) {
            Some(i) => i,
            None => break,
        };
        // If the next free block is after the current block,
        // that means that everything before is occupied, so we're done
        if next_free > i {
            break;
        }
        disk_map[next_free] = block.clone();
        disk_map[i] = DiskBlock::Free;
        last_free = next_free;
    }
}

fn checksum(disk_map: &Vec<DiskBlock>) -> Result<u64, Box<dyn Error>> {
    Ok(disk_map
        .iter()
        .enumerate()
        .map(|(i, b)| match b {
            DiskBlock::FileId(id) => {
                let index = u64::try_from(i)?;
                Ok(index * u64::from(*id))
            }
            DiskBlock::Free => Ok(0),
        })
        .collect::<Result<Vec<u64>, Box<dyn Error>>>()?
        .iter()
        .sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use day9::test_util::{
        string_to_disk_map,
        disk_map_to_string,
    };

    #[test]
    fn test_compact_disk_map() {
        let mut disk_map = string_to_disk_map("00...111...2...333.44.5555.6666.777.888899");
        compact_disk_map(&mut disk_map);
        assert_eq!(
            disk_map_to_string(&disk_map).unwrap(),
            "0099811188827773336446555566.............."
        );
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut disk_map = read_disk_map(&args.file, &mut |_, _, _| ()).unwrap();
    println!("Disk Map Length: {}", disk_map.len());
    compact_disk_map(&mut disk_map);
    println!("Checksum: {}", checksum(&disk_map).unwrap());
}
