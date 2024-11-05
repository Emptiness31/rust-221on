#[test]
fn test1() {
    let triangles = 6;
    let max_width = 2 * triangles + 1;
    (1..=triangles).for_each(|height| {
        (0..height).for_each(|i| {
            let space = max_width / 2 - i;
            let star = 2 * i + 1;
            println!("{}{}", " ".repeat(space), "*".repeat(star));
        });
    });
}