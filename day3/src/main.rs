use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    const NBITS: usize = 12;
    let filename = "../input";
    let file = File::open(filename).expect("Failed to read file");
    let reader = io::BufReader::new(file);
    let data: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let mut num_ones = [0; NBITS];
    let mut gamma = 0;

    for i in 0..NBITS {
        num_ones[i] = count_ones(&data, i);
    }

    // convert to number
    for (i, b) in num_ones.iter().enumerate() {
        gamma += if *b > (data.len() - b) {
            1 << (NBITS - 1 - i)
        } else {
            0
        };
    }
    // epsilon is bit inverse of gamma
    let epsilon = !gamma & ((1 << NBITS) - 1);

    println!("p1: {}", gamma * epsilon);

    // p2
    let mut o2_v = data.clone();
    for i in 0..NBITS {
        let ones = count_ones(&o2_v, i);
        let most_common = if ones >= (o2_v.len() - ones) {
            b'1'
        } else {
            b'0'
        };

        o2_v = o2_v
            .into_iter()
            .filter(|x| x.as_bytes()[i] == most_common)
            .collect();

        if o2_v.len() == 1 {
            break;
        }
    }
    let o2 = u32::from_str_radix(&(o2_v.pop().unwrap()), 2).unwrap();

    let mut co2_v = data.clone();
    for i in 0..NBITS {
        let ones = count_ones(&co2_v, i);
        let least_common = if ones >= (co2_v.len() - ones) {
            b'0'
        } else {
            b'1'
        };

        co2_v = co2_v
            .into_iter()
            .filter(|x| x.as_bytes()[i] == least_common)
            .collect();

        if co2_v.len() == 1 {
            break;
        }
    }

    let co2 = u32::from_str_radix(&(co2_v.pop().unwrap()), 2).unwrap();
    println!("p2: {}", o2 * co2);
}

fn count_ones(lines: &[String], pos: usize) -> usize {
    let mut ones = 0;
    for l in lines {
        if l.as_bytes()[pos] == b'1' {
            ones += 1;
        }
    }
    ones
}
