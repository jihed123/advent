use std::time::Instant;
use std::{fs, vec};
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
    let _color: Vec<&str> = vec!["red", "green", "blue"];

    for currentline in lines {
        // print!("{} ", currentline);

        let modifiedline: &str = &currentline[8..];
        // modifiedline = modifiedline.replace(" ", "");
        // print!("{} ", modifiedline);
        let linevec: Vec<&str> = modifiedline.split(|c| c == ';' || c == ',').collect();
        let mut maxred: i32 = 0;
        let mut maxgreen: i32 = 0;
        let mut maxblue: i32 = 0;
        for line in &linevec {
            if line.contains("red") {
                // print!("{} ", linevec[i]);
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect();

                for num in numbers {
                    if num > maxred {
                        maxred = num;
                    }
                }
            } else if line.contains("green") {
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect();

                for num in numbers {
                    if num > maxgreen {
                        maxgreen = num;
                    }
                }
            } else if line.contains("blue") {
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|word| word.parse::<i32>().ok())
                    .collect();

                for num in numbers {
                    if num > maxblue {
                        maxblue = num;
                    }
                }
            }
        }

        // println!("\nmax red : {} ", maxred);
        // println!("max green : {} ", maxgreen);
        // println!("max blue : {} ", maxblue);

        let s = currentline.split(":").collect::<Vec<&str>>()[0];
        // print!("{}", s);
        let game_number: i32 = s[5..].trim().parse().expect("Not a valid number");
        // println!("game number: {}", game_number);
        if maxred <= 12 && maxgreen <= 13 && maxblue <= 14 {
            number_final.push(game_number);
        }
    }
    // print!("Numbers: {:?}", number_final);
    let sum: i32 = number_final.iter().sum();
    println!("Sum: {}", sum);
    let duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
