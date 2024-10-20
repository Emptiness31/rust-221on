#[test]
fn test1() {
    const W: u32 = 20;
    const H: u32 = 10;
    let k = W / H; // 2
    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let id_diagonal = x == y * k;
            let id_co_diagonal = x == W - 1 - y * k;
            let c = if is_horizontal || is_vertical || id_diagonal || id_co_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}