use std::fs;
use std::time::Instant;
fn main() {
    let start = Instant::now();

    let duration = start.elapsed();
    println!("Time elapsed in whole program is: {:?}", duration);
}
