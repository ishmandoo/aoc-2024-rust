use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    println!("{}", solve_2());
}

fn solve_1() -> u32 {
    let mut sum = 0;
    for line in read_to_string("src/input.txt").unwrap().lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if safe_line(values) {
            sum += 1;
        }
    }

    sum
}

fn safe_line(values: Vec<i32>) -> bool {
    let mut last: Option<i32> = None;
    let mut ascending: Option<bool> = None;
    for val in values {
        match last {
            None => {
                last = Some(val);
            }
            Some(last_val) => {
                let diff = val - last_val;
                if ascending == None {
                    ascending = Some(val > last_val);
                }
                match ascending {
                    Some(true) => {
                        if (diff != 1) & (diff != 2) & (diff != 3) {
                            return false;
                        }
                    }
                    Some(false) => {
                        if (diff != -1) & (diff != -2) & (diff != -3) {
                            return false;
                        }
                    }
                    None => panic!(),
                }
                last = Some(val);
            }
        }
    }
    true
}

fn solve_2() -> u32 {
    let mut sum = 0;
    for line in read_to_string("src/input.txt").unwrap().lines() {
        let values: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let n = values.len();

        for i in 0..n {
            let mut values_clone = values.clone();
            values_clone.remove(i);
            if safe_line(values_clone) {
                sum += 1;
                break;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_line() {
        assert!(safe_line(vec![1, 2, 3, 4, 5]));
    }
}
