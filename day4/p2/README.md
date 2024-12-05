# 2024 Advent of Code Day 4

The solutions here are writting using an `ndarray`  of `u8` so I can experiment with an efficient representation of ASCII data.
In Rust, char is only guaraanteed to be smaller than a u32 which can result in a lot of wasted space for a big board.
