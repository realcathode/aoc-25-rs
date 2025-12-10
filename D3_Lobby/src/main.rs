use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    let reader = BufReader::new(file);
    let mut sum_part_1: u32 = 0;

    for i in reader.lines() {
        let line = i?;
        if line.is_empty() { continue; }

        let digits = line
            .bytes()
            .map(|b| b - b'0')
            .collect::<Vec<u8>>();

        if let line_max = two_piece_joltage(&digits) {
            sum_part_1 += line_max;
        }
    }

    println!("{}",sum_part_1);

    Ok(())
}

fn two_piece_joltage(digits: &Vec<u8>) -> u32 {
    let mut line_max = 0;
    let mut max_right = digits[digits.len() - 1];

    for &d in digits.iter().rev().skip(1) {
        let current_val = (d as u32 * 10) + (max_right as u32);

        if current_val > line_max {
            line_max = current_val;
        }

        if d > max_right {
            max_right = d;
        }
    }
    line_max
}
