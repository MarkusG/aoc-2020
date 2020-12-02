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

    let mut range = parts.next().unwrap().split("-");
    let lower = range.next().unwrap().parse::<i32>().unwrap();
    let upper = range.next().unwrap().parse::<i32>().unwrap();

    let letter = parts.next().unwrap().as_bytes()[0];
    let password = parts.next().unwrap();

    let mut count = 0;
    for &c in password.as_bytes() {
        if c == letter {
            count += 1;
        }
    }

    count >= lower && count <= upper
}

fn is_valid_2(line: &str) -> bool {
    let mut parts = line.split(" ");

    let mut pos = parts.next().unwrap().split("-");
    let first = pos.next().unwrap().parse::<usize>().unwrap() - 1;
    let second = pos.next().unwrap().parse::<usize>().unwrap() - 1;

    let letter = parts.next().unwrap().as_bytes()[0];
    let password = parts.next().unwrap().as_bytes();

    if password[first] == letter {
        return password[second] != letter
    }

    if password[second] == letter {
        return password[first] != letter
    }

    false
}
