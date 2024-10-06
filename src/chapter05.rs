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
    let x = 5;
    // Fill the blank
    let p = &x;
    println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}

#[test]
fn test11() {
    let x = 5;
    let y = &x;
    // Modify this line only
    assert_eq!(&x, y);
    println!("Success!");
}

#[test]
fn test12() {
    let s = String::from("hello, ");
    borrow_object(&s);
    println!("Success!");
}
fn borrow_object(_s: &String) {}

#[test]
fn test13() {
    let mut s = String::from("hello, ");
    push_str(&mut s);
    println!("Success!");
}
fn push_str(s: &mut String) {
    s.push_str("world")
}

#[test]
fn test14() {
    let mut s = String::from("hello, ");
    // Fill the blank to make it work
    let p = &mut s;
    p.push_str("world");
    println!("Success!");
}

#[test]
fn test15() {
    let c = '中';
    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));
    println!("Success!");
}
// Get memory address string
fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

#[test]
// Remove something to make it work
// Don't remove a whole line !
fn test16() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    println!("Success!");
}

#[test]
fn test17() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");
    borrow_object1(&mut s);
    println!("Success!");
}
fn borrow_object1(_s: &mut String) {}

#[test]
// This code has no errors!
//Ok: Borrow a mutable object as immutable
fn test18() {
    let mut s = String::from("hello, ");
    borrow_object2(&s);
    s.push_str("world");
    println!("Success!");
}
fn borrow_object2(_s: &String) {}

#[test]
// Comment one line to make it work
fn test19() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    //println!("{}",r1);
}

#[test]
fn test20() {
    let mut s = String::from("hello, ");
    let r1 = &mut s;
    let r2 = &mut s;
    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    println!("{}, {}", r1, r2);
}