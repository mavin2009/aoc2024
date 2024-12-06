pub fn solve() {
    use std::fs;

    let input = fs::read_to_string("input/day2.txt").expect("Failed to read input");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .collect()
        })
        .collect();

    // Chk if rpt safe
    let is_safe = |report: &[i32]| {
        report
            .windows(2)
            .all(|pair| (1..=3).contains(&(pair[1] - pair[0]).abs()))
            && (report.windows(2).all(|pair| pair[1] > pair[0])
                || report.windows(2).all(|pair| pair[1] < pair[0]))
    };

    // Pt 1: safe rpt cnt
    let part1_safe_count = reports.iter().filter(|&report| is_safe(report)).count();

    // Pt 2: dampener
    let part2_safe_count = reports
        .iter()
        .filter(|&report| {
            if is_safe(report) {
                return true;
            }
            // rem 1 lvl
            (0..report.len()).any(|i| {
                let mut modified_report = report.to_vec();
                modified_report.remove(i);
                is_safe(&modified_report)
            })
        })
        .count();

    println!("D2 - P1: Safe report count = {}", part1_safe_count);
    println!("D2 - P2: Safe report count = {}", part2_safe_count);
}
