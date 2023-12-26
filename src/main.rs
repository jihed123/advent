use std::fs;
use std::time::Instant;

fn main() {
    #![allow(unused_mut)]
    #![allow(unused_variables)]
    let start: Instant = Instant::now();

    // read file
    let filepath: &str = "./input3.txt";
    let contents: String = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    // i will probably need to make everything a array // hashmap? // vector?
    // with array i could use index to play with it
    // if it touch even one digit the whole number is valid
    // read first line
    let lines: std::str::Lines<'_> = contents.lines();
    let mut number_final: Vec<i32> = Vec::new();
    let mut part2: Vec<i32> = Vec::new();
    for currentline in lines.take(1) {
        print!("{} ", currentline);
    }

    let duration: std::time::Duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
