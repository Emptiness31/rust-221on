use rand::Rng;
fn random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_ind = 0;
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_ind = i;
        }
    }
    (min_sum, min_ind, min_ind + 1)
}
fn print_vector(data: &[i32]) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();
    print!("data:   [");
    for (i, value) in data.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        print!("{}", value);
    }
    println!("]");
    let (min_sum, ind1, ind2) = min_adjacent_sum(data);
    print!("indexes: ");
    for i in 0..data.len() {
        if i == ind1 {
            print!("\\__");
        } else if i == ind2 {
            print!("__/");
        } else {
            print!("    ");
        }
    }
    println!();
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[ind1], data[ind2], min_sum, ind1, ind2
    );
}
#[test]
fn test1() {
    let data = random_vector(20);
    print_vector(&data);
}