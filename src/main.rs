use std::fs;
use std::time::Instant;

fn main() {
    #![allow(unused_mut)]
    #![allow(unused_variables)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_assignments)]
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

    struct Number {
        number: Vec<i32>,
        start_index: i32,
        end_index: i32,
        line: i32,
    }

    // read first line
    let lines: std::str::Lines<'_> = contents.lines();

    let mut current_line: i32 = -1;

    let mut numbers: Vec<Number> = Vec::new();
    let mut current_number_on_array: i32 = -1;
    for currentline in lines.take(1) {
        print!("{} ", currentline);
        current_line += 1;
        // let mut number = Vec::new();
        let mut current_index: i32 = -1;
        let mut start_index: i32 = -1;
        let mut end_index: i32 = -1;
        let mut current_number: String = "".to_string();

        for char in currentline.chars() {
            current_index += 1;

            if char == '.' {
                if current_number == "" {
                    continue;
                } else {
                    end_index = current_index;
                    current_number_on_array += 1;
                    numbers.push(Number {
                        number: Vec::new(), // to fix
                        start_index: start_index,
                        end_index: end_index,
                        line: current_line,
                    });
                    current_number = "".to_string();
                }
            } else {
                if char.is_numeric() == true {
                    if current_number == "" {
                        start_index = current_index;
                        current_number = char.to_string();
                    } else {
                        current_number.push(char);
                    }
                }
            }
        }
        current_index = -1;
        start_index = -1;
        end_index = -1;
        current_number = "".to_string();
    }
    print!("\n{}\n", numbers[1].end_index);

    let duration: std::time::Duration = start.elapsed();
    println!("\nTime elapsed in whole program is: {:?}", duration);
}
