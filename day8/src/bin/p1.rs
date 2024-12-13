use clap::Parser;
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
        let antinode_dir1 = pos2.sub(&pos1);
        let antinode_dir2 = antinode_dir1.neg();
        let antinode_pos1 = pos2.add(&antinode_dir1);
        let antinode_pos2 = pos1.add(&antinode_dir2);

        let res: Vec<MapIndex> = [
            MapIndex::from_pos(&map, &antinode_pos1),
            MapIndex::from_pos(&map, &antinode_pos2),
        ]
        .into_iter()
        .filter_map(Result::ok)
        .collect();
        return Ok(res);
    })
    .unwrap();
    println!("num antinodes: {}", num_antinodes);
}
