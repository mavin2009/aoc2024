use std::fs;

fn can_part1(nums: &[i64], t: i64) -> bool {
    if nums.len() == 1 {
        return nums[0] == t;
    }
    let n = nums.len() - 1;
    for mask in 0..(1 << n) {
        let mut val = nums[0];
        for (i, &x) in nums[1..].iter().enumerate() {
            if (mask & (1 << i)) == 0 {
                val += x;
            } else {
                val *= x;
            }
        }
        if val == t {
            return true;
        }
    }
    false
}

fn concat_val(a: i64, b: i64) -> Option<i64> {
    let s = format!("{}{}", a, b);
    s.parse().ok()
}

fn dfs(nums: &[i64], i: usize, c: i64, t: i64, f: &mut bool) {
    if *f {
        return;
    }
    if i == nums.len() {
        if c == t {
            *f = true;
        }
        return;
    }
    let x = nums[i];
    dfs(nums, i + 1, c + x, t, f);
    if *f {
        return;
    }
    dfs(nums, i + 1, c * x, t, f);
    if *f {
        return;
    }
    if let Some(v) = concat_val(c, x) {
        dfs(nums, i + 1, v, t, f);
    }
}

pub fn solve() {
    let input = fs::read_to_string("input/day7.txt").unwrap();
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in input.lines() {
        let mut p = line.split(':');
        let t = p.next().unwrap().trim().parse::<i64>().unwrap();
        let nums = p
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let part1 = if can_part1(&nums, t) { t } else { 0 };
        sum1 += part1;
        if nums.len() == 1 {
            sum2 += (if nums[0] == t { t } else { 0 });
        } else {
            let mut found = false;
            dfs(&nums, 1, nums[0], t, &mut found);
            sum2 += (if found { t } else { 0 });
        }
    }
    println!("{}", sum1);
    println!("{}", sum2);
}
