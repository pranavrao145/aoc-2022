// Day 1 Problem 1

use std::{
    fs::File,
    io::{BufReader, Read},
};

fn find_max_calories(input: &mut String) -> i32 {
    return input
        .split("\n\n")
        .map(|chunk| {
            chunk
                .split("\n")
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .sum::<i32>()
        })
        .max()
        .unwrap();
}

fn main() {
    let mut reader = BufReader::new(File::open("input").unwrap());
    let mut input = String::new();

    reader
        .read_to_string(&mut input)
        .expect("Failed to parse input.");

    let mut input = input.trim().to_string();

    println!("{}", find_max_calories(&mut input));
}
