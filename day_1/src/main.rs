use std::fs;
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    println!("Hello, world!");
    println!("{}", solve_1());
    println!("{}", solve_2());
}

fn solve_1() -> u32 {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in read_to_string("src/input.txt").unwrap().lines() {
        let mut values = line.split_whitespace();
        left.push(values.next().unwrap().parse::<u32>().unwrap());
        right.push(values.last().unwrap().parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum: u32 = 0;

    let pairs = left.iter().zip(right.iter());
    for (x, y) in pairs {
        sum += x.abs_diff(*y);
    }

    sum
}

fn solve_1_gpt() -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = fs::read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            let mut values = line.split_whitespace().map(|v| v.parse::<u32>().unwrap());
            (values.next().unwrap(), values.last().unwrap())
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(&right).map(|(x, y)| x.abs_diff(*y)).sum()
}

fn solve_2() -> u32 {
    let mut left_map: HashMap<u32, u32> = HashMap::new();
    let mut right_map: HashMap<u32, u32> = HashMap::new();

    for line in read_to_string("src/input.txt").unwrap().lines() {
        let mut values = line.split_whitespace();

        let left_val = values.next().unwrap().parse::<u32>().unwrap();
        let right_val = values.next().unwrap().parse::<u32>().unwrap();

        match left_map.get(&left_val) {
            Some(count) => {
                left_map.insert(left_val, count + 1);
            }
            None => {
                left_map.insert(left_val, 1);
            }
        }

        match right_map.get(&right_val) {
            Some(count) => {
                right_map.insert(right_val, count + 1);
            }
            None => {
                right_map.insert(right_val, 1);
            }
        }
    }

    let mut sum: u32 = 0;

    for (left_val, count) in left_map.into_iter() {
        match right_map.get(&left_val) {
            Some(right_val) => sum += left_val * count * right_val,
            None => {}
        }
    }

    sum
}
