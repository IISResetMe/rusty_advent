use aocio::load_input;
use std::{env, str::FromStr, string::ParseError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return show_usage();
    }

    let day_parse_result = args[1].parse::<u8>();
    if day_parse_result.is_err() {
        show_usage();
        return;
    }

    match day_parse_result.unwrap() {
        1 => day_1(),
        2 => day_2(),
        _ => show_usage()
    }
}

fn show_usage() {
    println!("Expected number between 1 and 25: ");
    println!("  rusty_advent <1..25>");
}

fn day_1() {
    // io and prep
    let filepath = env::current_dir().unwrap().as_path().join("data\\day01.txt");
    let lines = load_input(filepath.into_os_string().to_str().unwrap());
    let measurements: Vec<i32> = lines.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();

    // part 1
    let mut count = 0;
    for idx in 1..measurements.len() {
        if measurements[idx] > measurements[idx - 1] {
            count = count + 1;
        }
    }

    println!("Part 1: {}", count);

    // part 2
    count = 0;
    let mut windows = measurements[..].windows(3); 
    let mut last: i32 = windows.next().unwrap()[0];
    for window in windows {
        let sum = window[0] + window[1] + window[2];
        if sum > last {
            count = count + 1;
        }
        last = sum;
    }

    println!("Part 2: {}", count);
}

fn day_2() {
    struct DiveInstruction {
        direction: Box<str>,
        distance: u32,
    }

    impl FromStr for DiveInstruction {
        type Err = ParseError;
        fn from_str(input: &str) -> Result<Self, Self::Err> {
            let mut chunks = input.split(" ");
            let direction = chunks.next().unwrap().into();
            let distance = chunks.next().unwrap().parse::<u32>().unwrap();

            return Ok(DiveInstruction { direction: direction, distance: distance });
        }
    }

    // io and prep
    let filepath = env::current_dir().unwrap().as_path().join("data\\day02.txt");
    let lines = load_input(filepath.into_os_string().to_str().unwrap());

    let instructions = lines.iter().map(|l| DiveInstruction::from_str(&l).unwrap());

    // part 1
    let mut x = 0;
    let mut y = 0;

    for instr in instructions {
        match (*instr.direction).as_ref() {
            "forward" => x = x + instr.distance,
            "down" => y = y + instr.distance,
            "up" => y = y - instr.distance,
            _ => ()
        }
    }

    println!("Part 1: {}", x * y);

    // part 2
    let mut aim =0;
    x = 0;
    y = 0;

    let instructions = lines.iter().map(|l| DiveInstruction::from_str(&l).unwrap());
    for instr in instructions {
        match (*instr.direction).as_ref() {
            "forward" => {x = x + instr.distance; y = y + aim * instr.distance},
            "down" => aim = aim + instr.distance,
            "up" => aim = aim - instr.distance,
            _ => ()
        }
    }

    println!("Part 2: {}", x * y);
}