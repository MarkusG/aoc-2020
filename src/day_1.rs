use std::fs;

pub fn day_1() {
    let input = fs::read_to_string("input/1.txt").expect("Error reading file");
    let entries = input.split("\n").map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    'outer: for x in &entries {
        for y in &entries {
            if x + y == 2020 {
                println!("First two: {}", x * y);
                break 'outer;
            }
        }
    }

    'outer: for x in &entries {
        for y in &entries {
            for z in &entries {
                if x + y + z == 2020 {
                    println!("First three: {}", x * y * z);
                    break 'outer;
                }
            }
        }
    }
}
