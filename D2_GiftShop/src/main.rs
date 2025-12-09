use std::io;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    let reader = BufReader::new(file);
    let mut sum:u64 = 0;

    for i in reader.lines() {
        let line = i?;
        if line.is_empty() { continue; }
        let ranges = line.split(",");

        for id_range in ranges {
            let parts = id_range.split('-').collect::<Vec<&str>>();
            let start = parts[0].parse::<u64>().unwrap();
            let end = parts[1].parse::<u64>().unwrap();

            for i in start..end + 1 {
                // length of a number = log10(num) + 1
                let length = i.ilog10() + 1;
                // only even numbers can be "mirrored" like this
                if length % 2 == 0 {
                    // 123123
                    // first three digits are i / 10*(len/2) = 123
                    // whole number is 123 * (10^(len/2) +1) = 123 * (1001) = 123123
                    if i == (i/10_u64.pow(length/2) * (10_u64.pow(length / 2)+1) ) {
                        sum += i;
                    }
                }
            }
        }
    }

    println!("{}", sum);
    Ok(())
}
