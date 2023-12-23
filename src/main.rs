use std::fs;
fn main() {
    // read file
    let filepath: &str = "./input1.txt";
    let contents: String =
        fs::read_to_string(filepath).expect("Something went wrong reading the file");

    // read first line
    let mut lines: std::str::Lines<'_> = contents.lines();
    let first_line: &str = lines.next().unwrap_or("None");
    println!("First line: {}", first_line);

    // make first line into a array of character so i can go through it
    let first_line_chars: Vec<char> = first_line.chars().collect();

    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..first_line_chars.len() {
        if first_line_chars[i].is_ascii_digit() {
            // add it to a array of numbers
            numbers.push(first_line_chars[i].to_digit(10).unwrap() as i32);

            print!("{} ", first_line_chars[i])
        }
    }

    // keep the first and last number in the array

    println!("\nNumbers: {:?}", numbers);

    println!("\nDone!");
}
