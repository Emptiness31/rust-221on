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