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
    let mut _number_final: Vec<i32> = Vec::new();
    let _color: Vec<&str> = vec!["red", "green", "blue"];

    for currentline in lines.take(1) {
        // print!("{} ", currentline);

        let modifiedline: &str = &currentline[8..];
        // modifiedline = modifiedline.replace(" ", "");
        // print!("{} ", modifiedline);
        let linevec: Vec<&str> = modifiedline.split(|c| c == ';' || c == ',').collect();
        let mut maxred: i32;
        let mut maxgreen: i32;
        let mut maxblue: i32;
        for i in 0..linevec.len() {
            if linevec[i].contains("red") {
                // print!("{} ", linevec[i]);
                for j in 0..linevec[i].len() {
                    if linevec[i].chars().nth(j).unwrap().is_ascii_digit() {
                        print!("{} ", linevec[i].chars().nth(j).unwrap());
                    }
                }
            }
            // print!("{} ", linevec[i]);
        }
        // println!("\nmodifiedline: {:?}", linevec[1]);

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
