use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<u64> = first_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    //let n = nums[0];
    let m = nums[1];

    let second_line = lines.next().unwrap().unwrap();
    let a_arr: Vec<u64> = second_line
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut remainder_count = vec![0u64; m as usize];
    remainder_count[0] = 1;

    let mut prefix = 0u64;

    for &num in &a_arr {
        prefix = (prefix + num) % m;
        remainder_count[prefix as usize] += 1;
    }

    let mut result = 0u64;
    for count in remainder_count {
        if count > 1 {
            result += count * (count - 1) / 2;
        }
    }
    println!("{}", result);
}