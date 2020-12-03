use std::fs;

pub fn day_3() {
    let input = fs::read_to_string("input/3.txt").expect("Error reading file");

    // Parse the map into a 2-dimensional vector
    let mut map = Vec::new();
    for (i, l) in input.split("\n").enumerate() {
        map.push(Vec::new());
        for &c in l.as_bytes() {
            map[i].push(c);
        }
    }

    println!("You hit {} trees in part 1", 
             count_collisions(3, 1, &map));
    println!("The product for part 2 is {}",
             count_collisions(1, 1, &map) * 
             count_collisions(3, 1, &map) * 
             count_collisions(5, 1, &map) * 
             count_collisions(7, 1, &map) * 
             count_collisions(1, 2, &map));
}

fn count_collisions(right: usize, down: usize, map: &Vec<Vec<u8>>) -> i32 {
    let width = map[0].len();
    let mut idx = 0;
    let mut count = 0;
    let mut itr = map.iter();

    // This must be what Auro was talking about with abusing iterators.
    // I like it.
    while let Some(l) = itr.next() {
        if l[idx] as char == '#' {
            count += 1;
        }
        idx = (idx + right) % width;
        if down > 1 {
            itr.nth(down - 2);
        }
    }

    count
}
