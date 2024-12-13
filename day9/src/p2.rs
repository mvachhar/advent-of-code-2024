use day9::{checksum, read_disk_map, DiskBlock};
use std::collections::HashMap;

use clap::Parser;

// Sucks that this bloats to 64 bits, but not worth custom struct

// This uses a dumb linear scan making the overall algorithm O(n^2)
// but it's good enough.  A better approach would be to have a data 
// structure to store free blocks and then use that to find the best block
// to allocate.  That would make the allocation O(1) or O(log n)and the
// compaction O(n) or O(n log n)
fn find_free_block(disk_map: &Vec<DiskBlock>, num_free: usize) -> Option<usize> {
    let mut free_info: Option<(usize, usize)> = None;
    for i in 0..disk_map.len() {
        free_info = match disk_map[i] {
            DiskBlock::Free => match free_info {
                Some((start, len)) =>  Some((start, len + 1)),
                None => Some((i, 1)),
            },
            DiskBlock::FileId(_) => None,
        };
        match free_info {
            Some((start, len)) => {
                if len == num_free {
                    return Some(start);
                }
            }
            None => {}
        }
    }
    return None;
}

fn compact_disk_map(disk_map: &mut Vec<DiskBlock>, file_info: &HashMap<u32, FileInfo>) -> () {
    let mut keys: Vec<&u32> = file_info.keys().collect();
    keys.sort_by(|a, b| b.cmp(a));
    for key in keys {
        
        let info = file_info.get(key).unwrap();
        let free_start = find_free_block(disk_map, info.len);
        match free_start {
            Some(start) => {
                if start < info.start {
                    for i in start..start + info.len {
                        disk_map[i] = DiskBlock::FileId(info.id);
                    }
                    for i in info.start..info.start + info.len {
                        disk_map[i] = DiskBlock::Free;
                    }
                }
            }
            None => continue, // No continguous free space found
        }
    }
}

#[derive(Debug, Clone)]
struct FileInfo {
    id: u32,
    start: usize,
    len: usize,
}

fn record_file_info(file_info: &mut HashMap<u32, FileInfo>, id: u32, start: usize, len: usize) {
    file_info.insert(id, FileInfo { id, start, len });
}

#[cfg(test)]
mod tests {
    use super::*;
    use day9::test_util::{disk_map_to_string, string_to_disk_map};

    fn disk_map_to_disk_info(disk_map: &Vec<DiskBlock>) -> HashMap<u32, FileInfo> {
        let mut file_info: HashMap<u32, FileInfo> = HashMap::new();
        let mut cur_file_info: Option<FileInfo> = None;
        for (i, block) in disk_map.iter().enumerate() {
            match block {
                DiskBlock::FileId(id) => match cur_file_info {
                    Some(mut info) => {
                        if info.id == *id {
                            info.len += 1;
                            cur_file_info = Some(info);
                        } else {
                            record_file_info(&mut file_info, info.id, info.start, info.len);
                            cur_file_info = Some(FileInfo {
                                id: *id,
                                start: i,
                                len: 1,
                            });
                        }
                    }
                    None => {
                        cur_file_info = Some(FileInfo {
                            id: *id,
                            start: i,
                            len: 1,
                        });
                    }
                },
                DiskBlock::Free => match cur_file_info {
                    Some(info) => {
                        record_file_info(&mut file_info, info.id, info.start, info.len);
                        cur_file_info = None;
                    }
                    None => {}
                },
            }
        }
        match cur_file_info {
            Some(info) => {
                record_file_info(&mut file_info, info.id, info.start, info.len);
            }
            None => {}
        }
        file_info
    }

    #[test]
    fn test_compact_disk_map() {
        let mut disk_map = string_to_disk_map("00...111...2...333.44.5555.6666.777.888899");
        let file_info = disk_map_to_disk_info(&disk_map);
        compact_disk_map(&mut disk_map, &file_info);
        assert_eq!(
            disk_map_to_string(&disk_map).unwrap(),
            "00992111777.44.333....5555.6666.....8888.."
        );
    }
}

#[derive(Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = Args::parse();
    let mut file_info: HashMap<u32, FileInfo> = HashMap::new();
    let mut disk_map = read_disk_map(&args.file, &mut |id, start, len| {
        record_file_info(&mut file_info, id, start, len)
    })
    .unwrap();
    println!("Disk Map Length: {}", disk_map.len());
    compact_disk_map(&mut disk_map, &mut file_info);
    println!("Checksum: {}", checksum(&disk_map).unwrap());
}
