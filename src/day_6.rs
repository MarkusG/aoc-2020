use std::fs;
use std::collections::HashMap;

pub fn day_6() {
    let input = fs::read_to_string("input/6.txt").expect("Error reading file");
    let groups = input.split("\n\n").collect::<Vec<&str>>();

    let count_yes_any: i32 = groups.iter().map(|g| num_yes_any(g)).sum();
    println!("Part one: {}", count_yes_any);
    let count_yes_all: i32 = groups.iter().map(|g| num_yes_all(g)).sum();
    println!("Part two: {}", count_yes_all);
}

fn num_yes_any(group: &str) -> i32 {
    let mut yes = HashMap::new();
    let mut itr = group.lines().flat_map(|s| s.chars());
    while let Some(c) = itr.next() {
        yes.entry(c).or_insert(true);
    }

    yes.len() as i32
}

fn num_yes_all(group: &str) -> i32 {
    let mut yes = HashMap::new();
    let mut itr = group.lines();
    let mut n_people = 0;
    while let Some(l) = itr.next() {
        n_people += 1;
        for c in l.chars() {
            let entry = yes.entry(c).or_insert(0);
            *entry += 1;
        }
    }

    yes.iter().map(|kv| *kv.1).filter(|q| *q == n_people).count() as i32
}
