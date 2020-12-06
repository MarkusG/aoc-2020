use std::fs;

pub fn day_5() {
    let input = fs::read_to_string("input/5.txt").expect("Error reading file");
    let mut ids = input.lines().map(|l| seat_id(l)).collect::<Vec<_>>();
    let max = ids.iter().max().unwrap();
    println!("Part one: {}", max);
    let mut last = *ids.iter().min().unwrap();
    ids.sort();
    let mut itr = ids.iter().skip(1);
    while let Some(&id) = itr.next() {
        if id == last + 1 {
            last = id;
        } else {
            println!("Part two: {}", last + 1);
            return
        }
    }
}

fn seat_id(id: &str) -> u32 {
    let row = group('F', 'B', &id[..7]);
    let col = group('L', 'R', &id[7..]);

    row * 8 + col
}

fn group(lower: char, upper: char, pos: &str) -> u32 {
    let mut u = 2_u32.pow(pos.len() as u32) - 1;
    let mut l = 0;
    let mut chars = pos.chars();
    while let Some(c) = chars.next() {
        let size = (u - l + 1) / 2;
        if c == upper {
            l += size;
        } else if c == lower {
            u -= size;
        }
    }

    u
}
