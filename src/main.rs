use aocio::load_input;
use std::env;

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