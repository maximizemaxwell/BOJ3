use std::io::{self, BufRead};

const MOD: i64 = 1000;

fn matrix_mul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>, n: usize) -> Vec<Vec<i64>> {
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % MOD
            }
        }
    }
    result
}

fn matrix_pow(matrix: &Vec<Vec<i64>>, pow: u64, n: usize) -> Vec<Vec<i64>> {
    if pow == 1 {
        let mut result = matrix.clone();
        for i in 0..n {
            for j in 0..n {
                result[i][j] %= MOD;
            }
        }
        return result;
    }
    let mut temp = matrix_pow(matrix, pow / 2, n);
    temp = matrix_mul(&temp, &temp, n);
    if pow % 2 == 1 {
        temp = matrix_mul(&temp, matrix, n);
    }
    temp
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut first_iter = first_line.split_whitespace();
    let n: usize = first_iter.next().unwrap().parse().unwrap();
    let b: u64 = first_iter.next().unwrap().parse().unwrap();

    let mut matrix = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        matrix.push(row);
    }

    let result = matrix_pow(&matrix, b, n);

    for row in result {
        let row_str = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        println!("{}", row_str);
    }
}