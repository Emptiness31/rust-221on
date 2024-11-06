#[test]
fn test_is_prime() {
    fn is_prime(n: &u32) -> bool {
        if *n < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= *n {
            if *n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];
    test_data
        .iter()
        .for_each(|(n, prime)|
            assert_eq!(is_prime(n), *prime)
        )
}