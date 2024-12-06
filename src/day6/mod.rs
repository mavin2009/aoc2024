use std::collections::{HashMap, HashSet};
use std::fs;

fn sim(mut g: Vec<Vec<char>>, sx: usize, sy: usize, sd: usize) -> Option<()> {
    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut v = HashSet::new();
    let (mut x, mut y, mut d) = (sx, sy, sd);
    v.insert((x, y, d));
    loop {
        let (dx, dy) = dirs[d];
        let nx = x as isize + dx;
        let ny = y as isize + dy;
        if nx < 0 || ny < 0 || nx >= g.len() as isize || ny >= g[0].len() as isize {
            return None;
        }
        if g[nx as usize][ny as usize] == '#' {
            d = (d + 1) % 4;
        } else {
            x = nx as usize;
            y = ny as usize;
            if !v.insert((x, y, d)) {
                return Some(());
            }
        }
    }
}

pub fn solve() {
    let g: Vec<Vec<char>> = fs::read_to_string("input/day6.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut sx = 0;
    let mut sy = 0;
    let (h, w) = (g.len(), g[0].len());
    let mut sd = 0;
    for i in 0..h {
        for j in 0..w {
            if "<>^v".contains(g[i][j]) {
                sx = i;
                sy = j;
                sd = "^>v<".chars().position(|c| c == g[i][j]).unwrap();
            }
        }
    }
    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if (i, j) != (sx, sy) && g[i][j] == '.' {
                let mut gg = g.clone();
                gg[i][j] = '#';
                if sim(gg, sx, sy, sd).is_some() {
                    count += 1;
                }
            }
        }
    }
    println!("{}", count);
}
