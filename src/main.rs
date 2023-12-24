use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();
    // read file
    let filepath: &str = "./input1.txt";
    let contents = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };
    // read first line
    let lines = contents.lines();
    let mut number_final: Vec<i32> = Vec::new();
    let number_tuples: Vec<(&str, usize)> = vec![
        ("nine", 9),
        ("eight", 8),
        ("seven", 7),
        ("six", 6),
        ("five", 5),
        ("four", 4),
        ("three", 3),
        ("two", 2),
        ("one", 1),
    ];
    for line in lines.clone() {
        // for line in lines.clone() {
        // let currentline: &str = lines.next().unwrap_or("None");
        let currentline: &str = line;
        // println!("line before modification: {}", currentline);

        // i have to check in the line if there is "nine, eight, seven, six, five, four, three, two, one, zero"

        let mut modifiedline: String = String::from(currentline);
        for (number_str, number_int) in &number_tuples {
            if modifiedline.contains(number_str) {
                // println!("number in letter: {}", number_str);
                modifiedline = modifiedline.replace(number_str, &number_int.to_string());
            }
        }
        let currentline_chars: Vec<char> = modifiedline.chars().collect();
        // println!("\nModified line: {}", modifiedline);
        let mut numbers: Vec<i32> = Vec::new();
        for i in 0..currentline_chars.len() {
            if currentline_chars[i].is_ascii_digit() {
                // add it to a array of numbers
                numbers.push(currentline_chars[i].to_digit(10).unwrap() as i32);

                // print!("{} ", currentline_chars[i])
            }
        }

        // keep the first and last number in the array
        if numbers.len() == 1 {
            let number = numbers[0] * 10 + numbers[0]; // Combine numbers directly
            number_final.push(number);
            // println!("\nModified line: {}", modifiedline);
        } else if numbers.len() >= 2 {
            let number = numbers[0] * 10 + numbers[numbers.len() - 1]; // Combine numbers directly
            number_final.push(number);
        } else {
            panic!("Something went wrong at x line");
        }
    }

    // println!("\nNumbers: {:?}", number_final);
    // give sum of number_final

    let sum: i32 = number_final.iter().sum();
    println!("Sum: {}", sum);
    println!("\nDone!");

    let duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
