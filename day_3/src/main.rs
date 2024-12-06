use regex::Regex;
use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    println!("{}", solve_2());
}

fn solve_1() -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut sum = 0;
    for line in read_to_string("src/input.txt").unwrap().lines() {
        let capture_matches = re.captures_iter(line);

        for capture_match in capture_matches.into_iter() {
            let a: u32 = capture_match.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = capture_match.get(2).unwrap().as_str().parse().unwrap();
            sum += a * b;
        }
    }
    sum
}

fn solve_2_re() -> u32 {
    let re_do = Regex::new(r"don\'t\(\).*?do\(\)").unwrap();

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut sum = 0;
    let input = read_to_string("src/input.txt").unwrap();
    let corrected_input = re_do.replace_all(&input, "");
    println!("{:?}", corrected_input);
    let capture_matches = re.captures_iter(&corrected_input);

    for capture_match in capture_matches.into_iter() {
        let a: u32 = capture_match.get(1).unwrap().as_str().parse().unwrap();
        let b: u32 = capture_match.get(2).unwrap().as_str().parse().unwrap();
        println!("{} {}", a, b);
        sum += a * b;
    }
    sum
}

fn solve_2() -> u32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let input = read_to_string("src/input.txt").unwrap();
    let capture_matches = re.captures_iter(&input);

    let mut mult = 1;
    for capture_match in capture_matches.into_iter() {
        println!("{:?}", capture_match);
        if capture_match.get(0).unwrap().as_str() == "don't()" {
            mult = 0;
        } else if capture_match.get(0).unwrap().as_str() == "do()" {
            mult = 1;
        } else {
            let a: u32 = capture_match.get(1).unwrap().as_str().parse().unwrap();
            let b: u32 = capture_match.get(2).unwrap().as_str().parse().unwrap();
            println!("{} {}", a, b);
            sum += a * b * mult;
        }
    }
    sum
}

//25073101
//29261624
//29261624
//98059175
//90669332
