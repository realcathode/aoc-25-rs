use std::io;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

struct Rotation {
    direction: char,
    value: i32,
}

const DIAL_SIZE: i32 = 100;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;

    let reader = io::BufReader::new(file);
    let mut position = 50;

    let mut password = 0;

    for i in reader.lines() {
        let line = i?;
        if line.is_empty() {
            continue;
        }

        let direction = line.chars().next().unwrap();
        let value = line[1..].parse::<i32>().unwrap();

        let rotation = Rotation{direction, value};
        // count zeros during rotation (task 2)
        let clicks = count_zero_clicks(position, direction, rotation.value);
        password += clicks;

        match rotation.direction {
            'L' => position = (position - rotation.value).rem_euclid(DIAL_SIZE),
            'R' => position = (position + rotation.value).rem_euclid(DIAL_SIZE),
            _ => panic!("Unknown direction"),
        }
        // count zeros after rotation (task 1)
        password += (position == 0) as i32;
    }
    println!("{}", password);
    Ok(())
}

fn count_zero_clicks(current_pos: i32, direction: char, distance: i32) -> i32 {
    if distance <= 0 { return 0; }

    // calculate how far to go to hit the first zero
    let d_to_first = match direction {
        'R' => if current_pos == 0 { DIAL_SIZE } else { DIAL_SIZE - current_pos },
        'L' => if current_pos == 0 { DIAL_SIZE } else { current_pos },
        _ => return 0,
    };

    if distance < d_to_first { return 0; }

    let remaining_distance = distance - d_to_first;
    1 + (remaining_distance / DIAL_SIZE)
}
