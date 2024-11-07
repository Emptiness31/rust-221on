#[test]
fn test1() {
    use std::io;
    // variable declaration //
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();
    // read variables
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    io::stdin().read_line(&mut _num_str_2).ok().expect("read error");
    // parse integers
    let mut _num_1 : i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let _num_2 : i32 = _num_str_2.trim().parse().ok().expect("parse error");
    // print the sum
    println!("{}", _num_1 + _num_2);
}

#[test]
fn test2() {
    use std::env;
    use std::fs::File;
    use std::io::{self, BufRead, Write};
    fn simple_array_sum(ar: &[i32]) -> i32 {
        let mut sum = 0;
        for &num in ar {
            sum += num;
        }
        sum
    }
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let _ar_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
    let ar: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let result = simple_array_sum(&ar);
    writeln!(&mut fptr, "{}", result).ok();
}
#[test]
fn count_apples_and_oranges1(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apple_count = 0;
    let mut orange_count = 0;
    for da in apples {
        let c = a + da;
        if c >= s && c <= t {
            apple_count += 1;
        }
    }
    for db in oranges {
        let c = b + db;
        if c >= s && c <= t {
            orange_count += 1;
        }
    }
    println!("{}\n{}", apple_count, orange_count);
}
fn count(center: i32, deltas: &[i32], l: i32, r: i32) -> i32 {
    let mut count = 0;
    for da in deltas {
        let c = center + da;
        if c >= l && c <= r {
            count += 1;
        }
    }
    count
}
fn count_apples_and_oranges2(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let apple_count = count(a, apples, s, t);
    let orange_count = count(b, oranges, s, t);
    println!("{}\n{}", apple_count, orange_count);
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */
fn main() {
    fn compare_triplets(a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut alice_score = 0;
        let mut bob_score = 0;
        for i in 0..3 {
            if a[i] > b[i] {
                alice_score += 1;
            } else if a[i] < b[i] {
                bob_score += 1;
            }
        }
        vec![alice_score, bob_score]
    }
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();
    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
    let a: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let b: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();
    let result = compare_triplets(&a, &b);
    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();
        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }
    writeln!(&mut fptr).ok();
}