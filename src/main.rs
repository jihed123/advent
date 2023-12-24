use std::fs;
fn main() {
    // read file
    let filepath: &str = "./input1.txt";
    let contents: String =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");

    // read first line
    let lines: std::str::Lines<'_> = contents.lines();
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
            let mut number_str: String = numbers[0].to_string();
            number_str.push_str(&numbers[0].to_string());
            number_final.push(number_str.parse::<i32>().unwrap());
            println!("\nModified line: {}", modifiedline);
            // print!("\n{} ", number_str);
        } else if numbers.len() >= 2 {
            let mut number_str: String = numbers[0].to_string();
            number_str.push_str(&numbers[numbers.len() - 1].to_string());
            number_final.push(number_str.parse::<i32>().unwrap());
            // print!("\n{} ", number_str);
        } else {
            panic!("Something went wrong at x line");
        }
    }

    // println!("\nNumbers: {:?}", number_final);
    // give sum of number_final

    let mut sum: i32 = 0;
    for i in 0..number_final.len() {
        sum += number_final[i];
    }
    println!("Sum: {}", sum);
    println!("\nDone!");
}
