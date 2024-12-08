use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
}

impl Line {
    fn full_line_points(&self, h: i64, w: i64) -> Vec<Point> {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        let g = gcd(dx.abs(), dy.abs());

        // Prevent division by zero
        if g == 0 {
            return vec![Point {
                x: self.x1,
                y: self.y1,
            }];
        }

        // Normalize direction
        let sx = dx / g;
        let sy = dy / g;

        let mut points = Vec::new();

        // Extend in the negative direction
        let mut x = self.x1;
        let mut y = self.y1;
        while x >= 0 && x < h && y >= 0 && y < w {
            points.push(Point { x, y });
            x -= sx;
            y -= sy;
        }

        // Reset to original position and move one step forward
        x = self.x1 + sx;
        y = self.y1 + sy;

        // Extend in the positive direction
        while x >= 0 && x < h && y >= 0 && y < w {
            points.push(Point { x, y });
            x += sx;
            y += sy;
        }

        points
    }
}

pub fn solve() {
    let grid: Vec<Vec<char>> = fs::read_to_string("input/day8.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let (h, w) = (grid.len() as i64, grid[0].len() as i64);
    let mut freq: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let c = grid[i as usize][j as usize];
            if c != '.' {
                freq.entry(c).or_default().push((i, j));
            }
        }
    }

    // Part 1
    let mut part1 = HashSet::new();
    for antennas in freq.values() {
        if antennas.len() < 2 {
            continue;
        }
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let (x1, y1) = antennas[i];
                let (x2, y2) = antennas[j];
                let p1 = (2 * x2 - x1, 2 * y2 - y1);
                if p1.0 >= 0 && p1.0 < h && p1.1 >= 0 && p1.1 < w {
                    part1.insert(Point { x: p1.0, y: p1.1 });
                }
                let p2 = (2 * x1 - x2, 2 * y1 - y2);
                if p2.0 >= 0 && p2.0 < h && p2.1 >= 0 && p2.1 < w {
                    part1.insert(Point { x: p2.0, y: p2.1 });
                }
            }
        }
    }

    // Part 2
    let mut part2 = HashSet::new();

    // Include all antenna positions as antinodes
    for antennas in freq.values() {
        if antennas.len() < 2 {
            continue;
        }
        for &(x, y) in antennas {
            part2.insert(Point { x, y });
        }
    }

    for antennas in freq.values() {
        if antennas.len() < 2 {
            continue;
        }

        // Iterate over all unique pairs of antennas
        for pair in antennas.iter().combinations(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];
            let line = Line {
                x1: *x1,
                y1: *y1,
                x2: *x2,
                y2: *y2,
            };

            // Add all points on the full line to part2
            let full_points = line.full_line_points(h, w);
            for point in full_points {
                part2.insert(point);
            }
        }
    }

    println!("Part 1: {}", part1.len());
    println!("Part 2: {}", part2.len());
}
