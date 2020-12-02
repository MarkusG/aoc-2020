use std::env;
use std::process;

use aoc_2020::day_1::day_1;
use aoc_2020::day_2::day_2;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("No day selected");
        process::exit(1);
    }

    match args[1].parse::<i32>().expect("Invalid day") {
        1 => day_1(),
        2 => day_2(),
        _ => ()
    }
}
