use regex::Regex;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/day3.txt").expect("Failed to read input");

    let instruction_re = Regex::new(
        r"(?x)                   # Enable verbose mode
        (mul\((\d+),(\d+)\))     # Capture mul(X,Y)
        |
        (do\(\))                 # Capture do()
        |
        (don't\(\))              # Capture don't()
        ",
    )
    .expect("Invalid regex");

    // P1: sum of valid `mul` instructions
    let part1_sum: i64 = instruction_re
        .captures_iter(&input)
        .filter_map(|caps| {
            if let Some(_) = caps.get(1) {
                // mul
                let x: i64 = caps[2].parse().ok()?;
                let y: i64 = caps[3].parse().ok()?;
                Some(x * y)
            } else {
                None
            }
        })
        .sum();

    println!("D3 - P1: Total sum of mul = {}", part1_sum);

    // P2: total sum of enabled `mul`
    let mut mul_enabled = true;
    let mut part2_sum = 0i64;

    for caps in instruction_re.captures_iter(&input) {
        if let Some(_) = caps.get(1) {
            if mul_enabled {
                let x: i64 = caps[2].parse().expect("Invalid number");
                let y: i64 = caps[3].parse().expect("Invalid number");
                part2_sum += x * y;
            }
        } else if let Some(_) = caps.get(4) {
            mul_enabled = true;
        } else if let Some(_) = caps.get(5) {
            mul_enabled = false;
        }
    }

    println!("D3 - P2: sum of enabled mul = {}", part2_sum);
}
