use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("../input").expect("Failed to read file");
    let reader = io::BufReader::new(file);

    let mut last = u32::MAX;
    let mut part1: u32 = 0;

    let mut window: [u32; 3] = [0; 3];
    let mut index = 0;
    let mut last_sum = u32::MAX;
    let mut part2: u32 = 0;

    for (i, line) in reader.lines().enumerate() {
        let n: u32 = line.unwrap().parse().expect("Failed to parse line");

        // part 1 calculation
        if n > last {
            part1 += 1;
        }
        last = n;

        // part 2 calculation
        window[index] = n;
        index = (index + 1) % 3;
        if i >= 2 {
            let sum: u32 = window.iter().sum();

            if sum > last_sum {
                part2 += 1;
            }
            last_sum = sum;
        }
    }

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
