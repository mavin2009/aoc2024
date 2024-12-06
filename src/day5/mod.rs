use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

pub fn solve() {
    // Read input from the file
    let input = fs::read_to_string("input/day5.txt").expect("Failed to read input");

    // Parse the input into ordering rules and updates
    let mut ordering_rules = Vec::new();
    let mut updates = Vec::new();
    let mut parsing_updates = false;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.contains(',') {
            parsing_updates = true;
        }
        if parsing_updates {
            updates.push(line.to_string());
        } else {
            ordering_rules.push(line.to_string());
        }
    }

    // Parse ordering rules into a data structure
    // We can build a mapping from X to a set of Y where X|Y
    let mut order_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in ordering_rules {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() != 2 {
            continue;
        }
        let x = parts[0].parse::<i32>().unwrap();
        let y = parts[1].parse::<i32>().unwrap();
        order_map.entry(x).or_insert_with(HashSet::new).insert(y);
    }

    // Part 1: Identify correctly ordered updates and sum their middle pages
    let mut correct_middle_pages = Vec::new();
    let mut incorrect_updates = Vec::new(); // For Part 2

    for update_line in &updates {
        let pages: Vec<i32> = update_line
            .split(',')
            .filter_map(|s| s.trim().parse::<i32>().ok())
            .collect();

        // Build a mapping from page number to its index in the update
        let mut page_positions: HashMap<i32, usize> = HashMap::new();
        for (idx, &page) in pages.iter().enumerate() {
            page_positions.insert(page, idx);
        }

        // For each applicable ordering rule, check if the order is correct
        let mut is_correct = true;

        for (&x, ys) in &order_map {
            if let Some(&pos_x) = page_positions.get(&x) {
                for &y in ys {
                    if let Some(&pos_y) = page_positions.get(&y) {
                        if pos_x >= pos_y {
                            // Rule violated
                            is_correct = false;
                            break;
                        }
                    }
                }
            }
            if !is_correct {
                break;
            }
        }

        if is_correct {
            // Get the middle page number
            let middle_idx = pages.len() / 2;
            let middle_page = pages[middle_idx];
            correct_middle_pages.push(middle_page);
        } else {
            // Collect incorrectly ordered updates for Part 2
            incorrect_updates.push(pages.clone());
        }
    }

    // Sum up the middle page numbers from correctly ordered updates
    let part1_total: i32 = correct_middle_pages.iter().sum();

    println!("Day 5 - Part 1: Sum of middle pages = {}", part1_total);

    // Part 2: Reorder incorrectly ordered updates and sum their middle pages
    let mut corrected_middle_pages = Vec::new();

    for pages in incorrect_updates {
        // Build a dependency graph for pages in the update
        let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut indegree: HashMap<i32, usize> = HashMap::new();

        let pages_set: HashSet<i32> = pages.iter().cloned().collect();

        // Initialize indegree for all pages to zero
        for &page in &pages {
            indegree.insert(page, 0);
        }

        // For each applicable ordering rule, build the graph
        for (&x, ys) in &order_map {
            if pages_set.contains(&x) {
                for &y in ys {
                    if pages_set.contains(&y) {
                        // Add edge from x to y
                        adj_list.entry(x).or_default().push(y);
                        *indegree.entry(y).or_insert(0) += 1;
                    }
                }
            }
        }

        // Perform topological sort using Kahn's algorithm
        let mut queue = VecDeque::new();
        for (&page, &deg) in &indegree {
            if deg == 0 {
                queue.push_back(page);
            }
        }

        let mut ordered_pages = Vec::new();

        while let Some(page) = queue.pop_front() {
            ordered_pages.push(page);

            if let Some(neighbors) = adj_list.get(&page) {
                for &neighbor in neighbors {
                    let deg = indegree.get_mut(&neighbor).unwrap();
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        // Check if we have a valid ordering
        if ordered_pages.len() != pages.len() {
            println!("Cycle detected in ordering rules for update {:?}", pages);
            continue; // Skip this update if there's a cycle (should not happen)
        }

        // Get the middle page number
        let middle_idx = ordered_pages.len() / 2;
        let middle_page = ordered_pages[middle_idx];
        corrected_middle_pages.push(middle_page);
    }

    // Sum up the middle page numbers from corrected updates
    let part2_total: i32 = corrected_middle_pages.iter().sum();

    println!(
        "Day 5 - Part 2: Sum of middle pages after correction = {}",
        part2_total
    );
}
