use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();

    // read file
    let filepath: &str = "./input2.txt";
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
    let color = vec!["red", "green", "blue"];

    for line in lines.take(1) {
        let currentline: &str = line;
        let mut modifiedline: String = String::from(currentline);
        print!("{} ", currentline);

        // modifiedline = modifiedline.replace("Game", "");
        modifiedline = modifiedline.drain(5..).collect::<String>();

        // v.split(' ').collect::<Vec<&str>>();
        // .join(" ");
        println!("\nmodifiedline: {:?}", modifiedline);

        // for color_str in &color {
        //     if modifiedline.contains(color_str) {
        //         modifiedline = modifiedline.replace(color_str, &color_str.len().to_string());
        //     }
        // }
        // let currentline_chars: Vec<char> = modifiedline.chars().collect();
        // let mut numbers: Vec<i32> = Vec::new();
        // for i in 0..currentline_chars.len() {
        //     if currentline_chars[i].is_ascii_digit() {
        //         numbers.push(currentline_chars[i].to_digit(10).unwrap() as i32);
        //     }
        // }

        // // keep the first and last number in the array
        // if numbers.len() == 1 {
        //     let number = numbers[0] * 10 + numbers[0]; // Combine numbers directly
        //     number_final.push(number);
        // } else if numbers.len() >= 2 {
        //     let number = numbers[0] * 10 + numbers[numbers.len() - 1]; // Combine numbers directly
        //     number_final.push(number);
        // }
    }

    let duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
