use std::fs;

pub fn solve() {
    // Read input from the file
    let input = fs::read_to_string("input/day4.txt").expect("Failed to read input");

    // Parse the grid into a 2D vector of chars
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    // Part 1: Find all occurrences of "XMAS"
    let word = "XMAS";
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();

    // Define the eight directions: (dx, dy)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut part1_count = 0;

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut match_found = true;
                for k in 0..word_len {
                    let x = i as isize + k as isize * dx;
                    let y = j as isize + k as isize * dy;
                    if x < 0 || x >= rows as isize || y < 0 || y >= cols as isize {
                        match_found = false;
                        break;
                    }
                    if grid[x as usize][y as usize] != word_chars[k] {
                        match_found = false;
                        break;
                    }
                }
                if match_found {
                    part1_count += 1;
                }
            }
        }
    }

    println!("Day 4 - Part 1: XMAS appears {} times", part1_count);

    // Part 2: Find all occurrences of "X-MAS" patterns
    // Just brute force this

    let mut part2_count = 0;

    for x in 0..rows {
        for y in 0..cols {
            if grid[x][y] == 'A' {
                let mut mas_diag1 = false;
                let mut mas_diag2 = false;

                // Check '\' diagonal
                if x >= 1 && y >= 1 && x + 1 < rows && y + 1 < cols {
                    let c1 = grid[x - 1][y - 1];
                    let c3 = grid[x + 1][y + 1];
                    if is_mas(c1, c3) {
                        mas_diag1 = true;
                    }
                }

                // Check '/' diagonal
                if x >= 1 && y + 1 < cols && x + 1 < rows && y >= 1 {
                    let c1_dash = grid[x - 1][y + 1];
                    let c3_dash = grid[x + 1][y - 1];
                    if is_mas(c1_dash, c3_dash) {
                        mas_diag2 = true;
                    }
                }

                // If both diagonals satisfy the 'MAS' pattern
                if mas_diag1 && mas_diag2 {
                    part2_count += 1;
                }
            }
        }
    }

    println!("Day 4 - Part 2: X-MAS appears {} times", part2_count);
}

// Helper function to check if the ends form "MAS" or "SAM"
fn is_mas(c1: char, c3: char) -> bool {
    (c1 == 'M' && c3 == 'S') || (c1 == 'S' && c3 == 'M')
}
