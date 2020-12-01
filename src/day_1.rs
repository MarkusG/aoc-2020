use std::fs;

pub fn day_1() {
    let input = fs::read_to_string("input/1.txt").expect("Error reading file");
    let entries = input.split("\n").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    if let Some(product) = search_two(&entries) {
        println!("First two: {}", product);
    }

    if let Some(product) = search_three(&entries) {
        println!("First three: {}", product);
    }
}

fn search_two(entries: &[i32]) -> Option<i32> {
    for x in entries {
        for y in entries {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }

    None
}

fn search_three(entries: &[i32]) -> Option<i32> {
    for x in entries {
        for y in entries {
            for z in entries {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }

    None
}
