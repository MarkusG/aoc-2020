// Yeah, I could have probably just used some regex patterns and had this
// done a lot more simply, but where's the fun in that? Also, who needs
// external crates?

use std::fs;

pub fn day_4() {
    let input = fs::read_to_string("input/4.txt").expect("Error reading file");
    let mut n_valid = input.split("\n\n").filter(|p| valid_fields(p)).count();
    println!("Part 1: {} valid passports", n_valid);

    n_valid = input.split("\n\n").filter(|p| valid_data(p)).count();
    println!("Part 2: {} valid passports", n_valid);
}

fn valid_fields(pp: &str) -> bool {
    ["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"]
        .iter()
        .all(|f| pp.contains(f))
}

fn valid_data(pp: &str) -> bool {
    if !valid_fields(pp) {
        return false
    }

    for mut kv in pp.split_whitespace().map(|kv| kv.split(":")) {
        let key = kv.next().unwrap();
        let val = kv.next().unwrap();
        match key {
            "byr" => if !validate_byr(val) { return false; }
            "iyr" => if !validate_iyr(val) { return false; }
            "eyr" => if !validate_eyr(val) { return false; }
            "hgt" => if !validate_hgt(val) { return false; }
            "hcl" => if !validate_hcl(val) { return false; }
            "ecl" => if !validate_ecl(val) { return false; }
            "pid" => if !validate_pid(val) { return false; }
            _ => ()
        }
    }

    true
}

fn validate_byr(byr: &str) -> bool {
    let b = byr.parse::<i32>().unwrap();
    b >= 1920 && b <= 2002
}

fn validate_iyr(iyr: &str) -> bool {
    let i = iyr.parse::<i32>().unwrap();
    i >= 2010 && i <= 2020
}

fn validate_eyr(eyr: &str) -> bool {
    let e = eyr.parse::<i32>().unwrap();
    e >= 2020 && e <= 2030
}

fn validate_hgt(hgt: &str) -> bool {
    // This technically isn't perfect - something like 100cmm or c10 matches,
    // but it works for our input so I don't care :)
    if let Some(idx) = hgt.find('c') {
        let n = 
            hgt.chars().take(idx).collect::<String>().parse::<i32>().unwrap();
        return n >= 150 && n <= 193
    } else if let Some(idx) = hgt.find('i') {
        let n = 
            hgt.chars().take(idx).collect::<String>().parse::<i32>().unwrap();
        return n >= 59 && n <= 76
    }
    false
}

fn validate_hcl(hcl: &str) -> bool {
    if let Some(idx) = hcl.find('#') {
        if idx != 0 {
            return false;
        }
    } else {
        return false;
    }

    let val = hcl.chars().skip(1).collect::<String>();
    return val.len() == 6 &&
        val.chars().all(|c| c.is_digit(16));
}

fn validate_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => return true,
        _ => return false
    }
}

fn validate_pid(pid: &str) -> bool {
    pid.chars().all(|c| c.is_digit(10)) && pid.chars().count() == 9
}
