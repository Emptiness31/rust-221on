#[test]
fn test1() {
    const SIZE: u8 = 11;
    let half = SIZE / 2;
    fn m(a: u8) -> u8 {
        SIZE - 1 - a
    }
    for y in 0..SIZE {
        for x in 0..SIZE {
            let q1 = x + y < half;
            let q2 = m(x) + y < half;
            let q3 = x + m(y) < half;
            let q4 = m(x) + m(y) < half;
            let c = if q1 || q2 || q3 || q4 { ' ' } else { '*' };
            print!("{c}");
        }
        println!();
    }
}