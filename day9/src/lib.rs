use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug, Clone, Copy)]
pub enum DiskBlock {
    FileId(u32),
    Free,
}

pub fn read_disk_map<F>(file: &str, on_file_id: &mut F) -> Result<Vec<DiskBlock>, io::Error>
where
    F: FnMut(u32, usize, usize) -> (),
{
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
        let file_id = next_file_id;
        let block = match cur_type {
            DiskBlock::FileId(_) => {
                next_file_id += 1;
                cur_type = DiskBlock::Free;
                DiskBlock::FileId(file_id)
            }
            DiskBlock::Free => {
                cur_type = DiskBlock::FileId(next_file_id);
                DiskBlock::Free
            }
        };
        let start = disk_map.len();
        let ulen = usize::try_from(len).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Disk map length overflow: {}", e),
            )
        })?;
        disk_map.extend(std::iter::repeat(block).take(ulen));
        on_file_id(file_id, start, ulen);
    }
    Ok(disk_map)
}

pub fn checksum(disk_map: &Vec<DiskBlock>) -> Result<u64, Box<dyn Error>> {
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

pub mod test_util {
    use super::*;

    pub fn string_to_disk_map(s: &str) -> Vec<DiskBlock> {
        s.chars()
            .map(|c| {
                if c == '.' {
                    DiskBlock::Free
                } else {
                    DiskBlock::FileId(c.to_digit(10).unwrap())
                }
            })
            .collect::<Vec<DiskBlock>>()
    }

    pub fn disk_map_to_string(disk_map: &Vec<DiskBlock>) -> Result<String, String> {
        disk_map
            .iter()
            .map(|b| match b {
                DiskBlock::FileId(id) => {
                    let res = id.to_string();
                    if res.len() > 1 {
                        return Err("File ID too large".to_string());
                    }
                    Ok(res)
                }
                DiskBlock::Free => Ok(".".to_string()),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::test_util::{disk_map_to_string, string_to_disk_map};
    use super::*;

    use std::io::Write;

    use tempfile::NamedTempFile;

    #[test]
    fn test_read_disk_map() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "2333133121414131402").unwrap();
        let path = file.path().to_str().unwrap();
        let disk_map = read_disk_map(path, &mut |_, _, _| ()).unwrap();
        assert_eq!(
            disk_map_to_string(&disk_map).unwrap(),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_checksum() {
        let disk_map = string_to_disk_map("0099811188827773336446555566..............");
        assert_eq!(checksum(&disk_map).unwrap(), 1928);
    }
}
