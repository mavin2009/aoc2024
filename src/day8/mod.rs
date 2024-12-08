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

pub fn solve() {
    let g: Vec<Vec<char>> = fs::read_to_string("input/day8.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let (h, w) = (g.len() as i64, g[0].len() as i64);
    let mut freq: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            let c = g[i as usize][j as usize];
            if c != '.' {
                freq.entry(c).or_default().push((i, j));
            }
        }
    }

    // Part 1: Only 2:1 ratio antinodes
    let mut part1 = HashSet::new();
    for (_, arr) in &freq {
        if arr.len() < 2 {
            continue;
        }
        for a_i in 0..arr.len() {
            for b_i in a_i + 1..arr.len() {
                let (x1, y1) = arr[a_i];
                let (x2, y2) = arr[b_i];
                let p1 = (2 * x2 - x1, 2 * y2 - y1);
                if p1.0 >= 0 && p1.0 < h && p1.1 >= 0 && p1.1 < w {
                    part1.insert(p1);
                }
                let p2 = (2 * x1 - x2, 2 * y1 - y2);
                if p2.0 >= 0 && p2.0 < h && p2.1 >= 0 && p2.1 < w {
                    part1.insert(p2);
                }
            }
        }
    }

    // Part 2: Any point on line formed by at least two same-frequency antennas
    let mut part2 = HashSet::new();
    for (_, arr) in freq {
        if arr.len() < 2 {
            continue;
        }
        // Antennas themselves
        for &a in &arr {
            part2.insert(a);
        }

        let mut lines_done = HashSet::new();
        for i in 0..arr.len() {
            for j in i + 1..arr.len() {
                let (x1, y1) = arr[i];
                let (x2, y2) = arr[j];
                let dx = x2 - x1;
                let dy = y2 - y1;
                let g = gcd(dx.abs(), dy.abs());
                let mut sx = dx / g;
                let mut sy = dy / g;
                if sx < 0 || (sx == 0 && sy < 0) {
                    sx = -sx;
                    sy = -sy;
                }
                // Find minimal anchor by shifting backward
                let mut ax = x1;
                let mut ay = y1;
                while ax - sx >= 0 && ax - sx < h && ay - sy >= 0 && ay - sy < w {
                    ax -= sx;
                    ay -= sy;
                }
                let line_key = (ax, ay, sx, sy);
                if lines_done.insert(line_key) {
                    let mut X = ax;
                    let mut Y = ay;
                    while X >= 0 && X < h && Y >= 0 && Y < w {
                        part2.insert((X, Y));
                        X += sx;
                        Y += sy;
                    }
                }
            }
        }
    }

    println!("{}", part1.len());
    println!("{}", part2.len());
}
