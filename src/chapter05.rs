#[test]
fn test1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}",x, y);
}

#[test]
// Don't modify code in test2!
fn test2() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}
// Only modify the code below!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

#[test]
fn test3() {
    let s = give_ownership();
    println!("{}", s);
}
// Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes(); //let _s = s.clone().into_bytes();?
    s
}

#[test]
fn test4() {
    let s = String::from("Hello World");
    print_str(s.clone());
    println!("{}", s);
}
fn print_str(s: String)  {
    println!("{}",s)
}

#[test]
// Don't use clone ,use copy instead
fn test5() {
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

#[test]
// make the necessary variable mutable
fn test6() {
    let s = String::from("Hello ");
    let mut s1 = s;
    s1.push_str("World!");
    println!("Success!");
}

#[test]
fn test7() {
    let x = Box::new(5);
    let mut y = Box::new(3);     // update this line, don't change other lines!
    *y = 4;
    assert_eq!(*x, 5);
    println!("Success!");
}

#[test]
fn test8() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

#[test]
fn test9() {
    let t = (String::from("hello"), String::from("world"));
    // Fill the blanks
    let (ref s1, ref s2) = t; //or let (s1, s2) = t.clone();
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

#[test]
fn test10() {

}