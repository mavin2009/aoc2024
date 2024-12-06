use std::collections::HashMap;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("input/day1.txt").expect("Failed to read input");

    // Parse input
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok());
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    // Part 1: compute total distance
    left.sort_unstable();
    right.sort_unstable();

    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("D1 - P1: Total distance = {}", total_distance);

    // Part 2: similarity score
    let right_counts: HashMap<i32, usize> = right.iter().fold(HashMap::new(), |mut map, &val| {
        *map.entry(val).or_insert(0) += 1;
        map
    });

    let similarity_score: i32 = left
        .iter()
        .map(|&val| val * (*right_counts.get(&val).unwrap_or(&0) as i32))
        .sum();
    println!("D1 - P2: Similarity score = {}", similarity_score);
}
