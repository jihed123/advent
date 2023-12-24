use std::fs;
use std::slice;
fn main() {
    // read file
    let filepath: &str = "./input1.txt";
    let contents: String =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");

    // read first line
    let lines: std::str::Lines<'_> = contents.lines();
    let mut number_final: Vec<i32> = Vec::new();
    for line in lines.clone().take(1) {
        // let currentline: &str = lines.next().unwrap_or("None");
        let currentline: &str = line;
        println!("First line: {}", currentline);

        // make first line into a array of character so i can go through it
        let currentline_chars: Vec<char> = currentline.chars().collect();

        // i have to check in the line if there is "nine, eight, seven, six, five, four, three, two, one, zero"

        let numberstr: Vec<&str> = vec![
            "nine", "eight", "seven", "six", "five", "four", "three", "two", "one", "zero",
        ];

        for i in 0..numberstr.len() {
            if currentline.contains(numberstr[i]) {
                // println!("{} is in the line", numberstr[i]);
                let slice: &str = &currentline[currentline.find(numberstr[i]).unwrap()..];
                println!("Slice: {}", slice);
            }
        }

        let mut numbers: Vec<i32> = Vec::new();
        for i in 0..currentline_chars.len() {
            if currentline_chars[i].is_ascii_digit() {
                // add it to a array of numbers
                numbers.push(currentline_chars[i].to_digit(10).unwrap() as i32);

                print!("{} ", currentline_chars[i])
            }
        }

        // keep the first and last number in the array
        if numbers.len() == 1 {
            number_final.push(numbers[0]);
            number_final.push(numbers[0]);
            // numbers.remove(0);
        } else if numbers.len() >= 2 {
            number_final.push(numbers[0]);
            number_final.push(numbers[numbers.len() - 1]);
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
