use std::fs;
use std::time::Instant;

fn main() {
    #![allow(unused_mut)]
    #![allow(unused_variables)]
    #![allow(dead_code)]
    #![allow(unused_imports)]
    #![allow(unused_assignments)]
    let start: Instant = Instant::now();
    print!("Hello, world!\n");
    // read file
    let filepath: &str = "c:/a/input3.txt";
    let contents: String = match fs::read_to_string(filepath) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Failed to read file1: {}", e);
            return;
        }
    };
    // i will probably need to make everything a array // hashmap? // vector?
    // with array i could use index to play with it
    // if it touch even one digit the whole number is valid

    struct Number {
        number: String,
        start_index: i32,
        end_index: i32,
        line: i32,
        count: Option<bool>,
    }

    struct Symbole {
        symbole: char,
        index: i32,
        line: i32,
    }

    let mut symboles: Vec<Symbole> = Vec::new();

    // read first line
    let lines: std::str::Lines<'_> = contents.lines();

    let mut current_line: i32 = -1;

    let mut numbers: Vec<Number> = Vec::new();
    let mut current_number_on_array: i32 = -1;
    for currentline in lines.take(7) {
        print!("{} ", currentline);
        current_line += 1;
        // let mut number = Vec::new();
        let mut current_index: i32 = -1;
        let mut start_index: i32 = -1;
        let mut end_index: i32 = -1;
        let mut current_number: String = "".to_string();

        for char in currentline.chars() {
            current_index += 1;

            if char.is_numeric() == true {
                if current_number == "" {
                    start_index = current_index;
                    current_number = char.to_string();
                } else {
                    current_number.push(char);
                }
            } else {
                if char != '.' {
                    symboles.push(Symbole {
                        symbole: char,
                        index: current_index,
                        line: current_line,
                    });
                }
                if current_number != "" {
                    end_index = current_index - 1;
                    current_number_on_array += 1;
                    numbers.push(Number {
                        number: current_number.clone(), // to fix
                        start_index: start_index,
                        end_index: end_index,
                        line: current_line,
                        count: None,
                    });

                    current_number = "".to_string();
                }
            }
        }

        //     if char == '.' {
        //         if current_number == "" {
        //             continue;
        //         } else {
        //             end_index = current_index - 1;
        //             current_number_on_array += 1;
        //             numbers.push(Number {
        //                 number: current_number, // to fix
        //                 start_index: start_index,
        //                 end_index: end_index,
        //                 line: current_line,
        //                 count: None,
        //             });
        //             current_number = "".to_string();
        //         }
        //     } else {
        //         if char.is_numeric() == true {
        //             if current_number == "" {
        //                 start_index = current_index;
        //                 current_number = char.to_string();
        //             } else {
        //                 current_number.push(char);
        //             }
        //         } else {
        //             symboles.push(Symbole {
        //                 symbole: char,
        //                 index: current_index,
        //                 line: current_line,
        //             });
        //             // end_index = current_index - 1;
        //             // current_number_on_array += 1;
        //             // current_number = "".to_string();
        //         }
        //     }
        // }
        current_index = -1;
        start_index = -1;
        end_index = -1;
        current_number = "".to_string();
    }

    for symbole in symboles.iter() {
        for number in numbers.iter() {
            if symbole.line == number.line {
                // if number.line == 6 && symbole.symbole == '*' {
                //     // print!("{} ", number.number);
                // }

                if (symbole.index - 1 >= number.start_index
                    && symbole.index - 1 == number.end_index)
                    || (symbole.index + 1 == number.start_index
                        && symbole.index + 1 <= number.end_index)
                {
                    if number.count == None {
                        print!("\n{} ", number.number);
                        print!("{} ", symbole.symbole);
                        // number.count = Some(true);
                    } else {
                        // number.count = Some(false);
                    }
                }
            }
        }
    }
    // for number in numbers.iter() {
    //     if number.count == None && number.line == 6 {
    //         print!("{} ", number.number);
    //     }
    // }

    // print!("\n{}", symboles[6].symbole);

    let duration: std::time::Duration = start.elapsed();
    println!("\nTime elapsed in whole program is: {:?}", duration);
}
