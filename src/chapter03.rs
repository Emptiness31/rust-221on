
#[test]
fn test() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}

#[test]
fn test2() {
    let mut x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("Success!");
}

#[test]
fn test3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
}

#[test]
fn test4() {
    define_x();
}
fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

#[test]
fn test5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x); // Prints "42".
}

// Remove a line in the code to make it compile
#[test]
fn test6() {
    let mut x: i32 = 1; //warning: variable `x` is assigned to, but never used
    x = 7;
    x += 3; //warning: value assigned to `x` is never read
    let _y = 4;
    // Shadowing
    let _y = "I can also be bound to text!";
    println!("Success!");
}

#[test]
fn test7() {
    let _x = 1;
}

#[test]
fn test8() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
    println!("Success!");
}

#[test]
fn test9() {
    let (x, y);
    (x,..) = (3, 4); //працює, але показує як помилка, замінити ..?
    [.., y] = [1, 2];
    // Fill the blank to make the code work
    assert_eq!([x,y], [3, 2]);
    println!("Success!");
}