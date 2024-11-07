#[test]
fn test1() {
    fn rotate2(s: &str, n: &isize) -> String {
        let len = s.len() as isize;
        if len == 0 {
            return s.to_string();
        }
        let shift = ((*n % len) + len) % len;
        let part1 = &s[(len - shift) as usize..];
        let part2 = &s[..(len - shift) as usize];
        let result = format!("{}{}", part1, part2);
        result
    }
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];
    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate2(s, n),
                exp.to_string()
            )
        );
}