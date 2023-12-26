use std::fs;
use std::time::Instant;

fn main() {
    #![allow(unused_mut)]
    #![allow(unused_variables)]
    let start: Instant = Instant::now();

    // read file
    let filepath: &str = "./input2.txt";
    let contents: String = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    // read first line
    let lines: std::str::Lines<'_> = contents.lines();
    let mut number_final: Vec<i32> = Vec::new();
    let mut part2: Vec<i32> = Vec::new();
    for currentline in lines {
        // print!("{} ", currentline);

        let modifiedline: &str = &currentline[8..];
        // modifiedline = modifiedline.replace(" ", "");
        // print!("{} ", modifiedline);
        let linevec: Vec<&str> = modifiedline.split(|c: char| c == ';' || c == ',').collect();
        let mut maxred: i32 = 0;
        let mut maxgreen: i32 = 0;
        let mut maxblue: i32 = 0;
        let mut minred: Option<i32> = None;
        let mut mingreen: Option<i32> = None;
        let mut minblue: Option<i32> = None;
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
                    if let Some(min) = minred {
                        if num < min {
                            minred = Some(num);
                        }
                    } else {
                        minred = Some(num);
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
                    if let Some(min) = mingreen {
                        if num < min {
                            mingreen = Some(num);
                        }
                    } else {
                        mingreen = Some(num);
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
                    if let Some(min) = minblue {
                        if num < min {
                            minblue = Some(num);
                        }
                    } else {
                        minblue = Some(num);
                    }
                }
            }
        }

        // println!("\nmax red : {} ", maxred);
        // println!("max green : {} ", maxgreen);
        // println!("max blue : {} ", maxblue);

        match minred {
            // Some(value) => println!("min red : {} ", value),
            Some(_) => {}
            None => panic!("min red is None"),
        }
        match mingreen {
            // Some(value) => println!("min green : {} ", value),
            Some(_) => {}
            None => panic!("min green is None"),
        }
        match minblue {
            // Some(value) => println!("min blue : {} ", value),
            Some(_) => {}
            None => panic!("min blue is None"),
        }

        let s = currentline.split(":").collect::<Vec<&str>>()[0];

        let game_number: i32 = s[5..].trim().parse().expect("Not a valid number");
        if maxred <= 12 && maxgreen <= 13 && maxblue <= 14 {
            number_final.push(game_number);
        }

        // i just realized i didnt need the min stuff for part 2

        let power = maxred * maxgreen * maxblue;
        part2.push(power);

        // if minred.is_some() && mingreen.is_some() && minblue.is_some() {
        //     let power = minred.unwrap() * mingreen.unwrap() * minblue.unwrap();
        //     part2.push(power);
        // } else {
        //     panic!("min red or min green or min blue is None")
        // }
    }
    // print!("Numbers: {:?}", number_final);
    // for part 1 only
    let sum: i32 = number_final.iter().sum();
    // for part 2
    let sum2: i32 = part2.iter().sum();

    println!("Sum1: {}", sum);
    println!("Sum2: {}", sum2);
    let duration: std::time::Duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
