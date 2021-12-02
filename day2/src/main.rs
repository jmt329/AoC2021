use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("../input").expect("Failed to read file");
    let reader = io::BufReader::new(file);

    let mut x = 0;
    let mut y = 0;
    let mut y2 = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let n = line.unwrap();
        let first = n.as_bytes()[0];

        let last = last_word(&n);

        if first == b'f' {
            x += last;
            y2 += aim * last;
        } else if first == b'd' {
            y += last;
            aim += last;
        } else if first == b'u' {
            y -= last;
            aim -= last;
        }
    }
    println!("Part 1: {}", x * y);
    println!("Part 2: {}", x * y2);
}

fn last_word(s: &String) -> u32 {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate().rev() {
        if item == b' ' {
            return s[i + 1..].parse::<u32>().unwrap();
        }
    }

    s[..].parse::<u32>().unwrap()
}
