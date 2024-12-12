# Day 6 2024 Advent of Code

This is a Rust implementation of the solutions to the 2024 Advent of Code Day 6 puzzles.

Unlike prior solutions, this one uses a common library for p1 and p2.  
This let's me learn how to build a library and use it in a main program.
It also lets me experiment with generics, traits, and other Rust features related to abstraction.

For p2, I purposely made the BoardState enum not implement Copy so that I could experiment with ownership and borrowing in Rust.
