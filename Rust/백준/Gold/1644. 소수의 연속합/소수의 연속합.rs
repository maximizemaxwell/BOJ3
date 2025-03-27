use std::io::{self, BufRead};

fn sieve(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    if n >= 0 {
        is_prime[0] = false;
    }
    if n >= 1 {
        is_prime[1] = false;
    }
    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..=n).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    (2..=n).filter(|&x| is_prime[x]).collect()
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let primes = sieve(n);

    let mut count = 0;
    let mut sum = 0;
    let mut left = 0;
    let mut right = 0;

    while right <= primes.len() {
        if sum >= n {
            if sum == n {
                count += 1;
            }
            sum -= primes[left];
            left += 1;
        } else {
            if right == primes.len() {
                break;
            }
            sum += primes[right];
            right += 1;
        }
    }
    println!("{}", count);
}