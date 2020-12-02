use std::fs;

pub fn day_2() {
    let input = fs::read_to_string("input/2.txt").expect("Error reading file");
    
    let n_invalid = input.split("\n").filter(|l| is_valid_1(&l)).count();
    println!("{} passwords are invalid for part 1", n_invalid);
    let n_invalid = input.split("\n").filter(|l| is_valid_2(&l)).count();
    println!("{} passwords are invalid for part 2", n_invalid);
}

fn is_valid_1(line: &str) -> bool {
    let mut parts = line.split(" ");

    let mut range = parts.nth(0).unwrap().split("-");
    let lower = range.nth(0).unwrap().parse::<i32>().unwrap();
    let upper = range.nth(0).unwrap().parse::<i32>().unwrap();

    let letter = &parts.nth(0).unwrap().chars().nth(0).unwrap();
    let password = parts.nth(0).unwrap();

    let mut count = 0;
    for c in password.chars() {
        if c == *letter {
            count += 1;
        }
    }

    count >= lower && count <= upper
}

fn is_valid_2(line: &str) -> bool {
    let mut parts = line.split(" ");

    let mut pos = parts.nth(0).unwrap().split("-");
    let first = pos.nth(0).unwrap().parse::<usize>().unwrap() - 1;
    let second = pos.nth(0).unwrap().parse::<usize>().unwrap() - 1;

    let letter = parts.nth(0).unwrap().as_bytes()[0];
    let password = parts.nth(0).unwrap().as_bytes();

    if password[first] == letter {
        return password[second] != letter
    }

    if password[second] == letter {
        return password[first] != letter
    }

    false
}
