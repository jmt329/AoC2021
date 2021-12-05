use std::{fs, str::Lines};

#[derive(Clone, Copy, Debug)]
struct BingoTile {
    number: u8,
    hit: bool,
}

#[derive(Debug)]
struct BingoBoard {
    tiles: [BingoTile; 25],
    won: bool,
}

impl BingoBoard {
    fn new(row_vectors: Vec<&str>) -> BingoBoard {
        let t = BingoTile {
            number: 0,
            hit: false,
        };
        let mut b = BingoBoard {
            tiles: [t; 25],
            won: false,
        };

        let row_strs = row_vectors.into_iter().flat_map(|x| x.split_whitespace());
        for (i, n) in row_strs.enumerate() {
            b.tiles[i].number = n.parse().unwrap();
        }

        b
    }

    fn check_row(&self, r: usize) -> bool {
        let start = r * 5;
        let end = start + 5;
        for c in start..end {
            if !self.tiles[c].hit {
                return false;
            }
        }
        true
    }

    fn check_col(&self, c: usize) -> bool {
        for r in 0..5 {
            if !self.tiles[(r * 5) + c].hit {
                return false;
            }
        }
        true
    }

    fn check_win(&self) -> bool {
        for r in 0..5 {
            if self.check_row(r) {
                return true;
            }
        }
        for c in 0..5 {
            if self.check_col(c) {
                return true;
            }
        }
        false
    }

    fn call_num(&mut self, n: u8) {
        for mut t in &mut self.tiles {
            if t.number == n {
                t.hit = true;
            }
        }
    }

    fn calc_score(&self) -> u32 {
        let mut score: u32 = 0;
        for t in self.tiles {
            if !t.hit {
                score += t.number as u32;
            }
        }
        score
    }
}

fn main() {
    let file_string = fs::read_to_string("../input").unwrap();
    let mut file_lines = file_string.lines();

    let numbers: Vec<u8> = file_lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let mut boards = get_boards(&mut file_lines);
    let num_boards = boards.len();
    let mut boards_won = 0;
    let mut hit_winner = false;

    for n in numbers {
        for b in &mut boards {
            b.call_num(n);
            if b.check_win() {
                if !b.won {
                    // first time this board won
                    b.won = true;
                    boards_won += 1;
                    if boards_won == num_boards {
                        // last board to win
                        println!("p2: {}", n as u32 * b.calc_score());
                    }
                }

                if !hit_winner {
                    // first board to win
                    println!("p1: {}", n as u32 * b.calc_score());
                }
                hit_winner = true;
            }
        }
    }
}

fn get_boards(lines: &mut Lines) -> Vec<BingoBoard> {
    let mut boards: Vec<BingoBoard> = vec![];

    // new line after numbers and between boards
    while lines.next().is_some() {
        let board: Vec<&str> = lines.take(5).collect();
        boards.push(BingoBoard::new(board));
    }
    boards
}
