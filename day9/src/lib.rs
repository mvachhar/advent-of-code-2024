use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug, Clone, Copy)]
pub enum DiskBlock {
    FileId(u32),
    Free,
}

pub fn read_disk_map(file: &str) -> Result<Vec<DiskBlock>, io::Error> {
    let file = File::open(file).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 1];
    let mut disk_map = Vec::new();

    let mut next_file_id: u32 = 0;
    let mut cur_type = DiskBlock::FileId(0);
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        };
        let byte = buffer[0];
        let len = match byte {
            b'\n' => {
                break;
            }
            b'0'..=b'9' => u32::from(byte - b'0'),
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("Invalid byte {}", byte),
                ));
            }
        };
        let block = match cur_type {
            DiskBlock::FileId(_) => {
                let file_id = next_file_id;
                next_file_id += 1;
                cur_type = DiskBlock::Free;
                DiskBlock::FileId(file_id)
            }
            DiskBlock::Free => {
                cur_type = DiskBlock::FileId(next_file_id);
                DiskBlock::Free
            }
        };
        disk_map.extend(std::iter::repeat(block).take(len as usize));
    }
    Ok(disk_map)
}
