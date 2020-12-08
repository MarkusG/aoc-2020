use std::fs;
use std::collections::{HashMap, HashSet};

pub fn day_7() {
    let input = fs::read_to_string("input/7.txt").expect("Error reading file");
    let lines = input.lines().collect::<Vec<_>>();
    let color = "shiny gold";

    let map = parse(&lines);
    let mut can_hold = HashSet::new();
    containers(&map, &mut can_hold, color.to_string());
    println!("Part one: {}", can_hold.len());
}

fn parse(lines: &[&str]) -> HashMap<String, Vec<(String, i32)>> {
    let mut result = HashMap::new();
    for l in lines {
        // I don't like this but it works
        let words = l.split_whitespace().collect::<Vec<_>>();
        let rule_color = words[..2].join(" ");
        result.insert(rule_color.clone(), Vec::new());
        for (i, &w) in words.iter().enumerate() {
            if ["bag,", "bags,", "bag.", "bags."].contains(&w) {
                let count = words[i - 3].parse::<i32>();
                let color = words[i - 2..i].join(" ");
                if let Ok(i) = count {
                    result.get_mut(&rule_color).unwrap().push((color, i));
                }
            }
        }
    }

    result
}

fn containers(map: &HashMap<String, Vec<(String, i32)>>,
              result: &mut HashSet<String>,
              color: String) {
    for (k, v) in map.iter() {
        // I should really learn how to properly use this language
        if v.iter().map(|(col, _)| col).collect::<Vec<_>>().contains(&&color) {
            result.insert(k.to_string());
            containers(map, result, k.to_string());
        }
    }
}
