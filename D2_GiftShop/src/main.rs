use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    let reader = BufReader::new(file);

    let mut sum_part_1: u64 = 0;
    let mut sum_part_2: u64 = 0;

    for i in reader.lines() {
        let line = i?;
        if line.is_empty() { continue; }
        let ranges = line.split(",");

        for id_range in ranges {
            let parts = id_range.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            for n in start..=end {
                if is_strictly_halved(n) {
                    sum_part_1 += n;
                }

                if is_repetitive(n) {
                    sum_part_2 += n;
                }
            }
        }
    }

    println!("Part 1 Sum: {}", sum_part_1);
    println!("Part 2 Sum: {}", sum_part_2);

    Ok(())
}


// example 1717
// 17 * 1000 + 17 = 1717 <=>
// 17 * 1001 = 1717
// num == top_half * (10^(len/2) + 1)
fn is_strictly_halved(n: u64) -> bool {
    let length = n.ilog10() + 1;
    if length % 2 != 0 {
        return false;
    }

    let shift = 10_u64.pow(length / 2);
    n == (n / shift) * (shift + 1)
}

// example with num = 69696969 (length 8):
// try pattern length = 2 (checking for 69 repeated k times)
// k = 4
// multiplier = 1+100+10000+1000000=1010101
// check if 69 * 1010101 = num
fn is_repetitive(n: u64) -> bool {
    let length = n.ilog10() + 1;

    // check all valid pattern lengths p
    // a pattern must repeat at least twice, max p is length / 2
    for p in 1..=(length / 2) {
        if length % p == 0 {
            let k = length / p; // number of repetitions
            let block_shift = 10_u64.pow(p);

            let mut multiplier = 0;
            for _ in 0..k {
                multiplier = multiplier * block_shift + 1;
            }

            let pattern = n / 10_u64.pow(length - p);

            // check if pattern reconstructs the number
            if pattern * multiplier == n {
                return true;
            }
        }
    }
    false
}