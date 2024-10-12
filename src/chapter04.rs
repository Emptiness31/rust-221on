/// https://practice.course.rs/basic-types/numbers.html
#[test]
fn test41() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z: i32 = 10; // Type of z ?
    println!("Success!");
}

#[test]
fn test42() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test43() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test44() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}

#[test]
fn test45() {
    let v1 = 251_u16 + 8;
    let v2 = i16::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
}

#[test]
fn test46() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255
    assert_eq!(v, 1597);
    println!("Success!");
}

// Fill the blank to make it work
#[test]
fn test47() {
    let x = 1_000.000_1; // f64
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64
    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}
fn type_of1<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test48() {
    assert_eq!(0.1_f32 + 0.2_f32, 0.3_f32); //or as f32
    println!("Success!");
}

#[test]
fn test49() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert_eq!(sum, -5); //assert! (.. == ..)
    for c in 'a'..='z' {
        println!("{}",c as u8);
    }
}
use std::ops::{Range, RangeInclusive};
#[test]
fn test410() {
    assert_eq!(1..5, Range{ start: 1, end: 5 }); //була проблема з непотрібними дужками лол
    assert_eq!(1..=5, RangeInclusive::new(1, 5)); //
    println!("Success!");
}

#[test]
fn test411() {
    // Integer addition
    assert_eq!(1u32 + 2u32, 3u32);
    // Integer subtraction
    assert_eq!(1i32 - 2i32, -1i32);
    assert_eq!(1i8 - 2i8, -1i8); // з 1u8 в і8
    assert_eq!(3 * 50, 150);
    assert_eq!(9.6_f32 / 3.2_f32, 3.0_f32); // error ! make it work
    assert_eq!(24 % 5, 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);
    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}
/// https://practice.course.rs/basic-types/char-bool-unit.html
#[test]
fn test412() {
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1),4); // 4байта
    let c2 = '中';
    assert_eq!(size_of_val(&c2),4);
    println!("Success!");
}

#[test]
fn test413() {
    let c1 = '中'; //" " в ' '
    print_char(c1);
}
fn print_char(c : char) {
    println!("{}", c);
}

#[test]
fn test414() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("Success!");
    }
}

#[test]
fn test415() {
    let f = false; //було тру
    let t = false; //було let t = true && false;
    assert_eq!(t, f);
    println!("Success!");
}

#[test]
fn test416() {
    let _v: () = ();
    let _s = (2, 3); //not _v? _v:(i32, i32) помилка
    assert_eq!(_v, implicitly_ret_unit());
    println!("Success!");
}
fn implicitly_ret_unit() {
    println!("I will return a ()");
}

#[test]
fn test417() {
    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);
    println!("Success!");
}
/// https://practice.course.rs/basic-types/statements-expressions.html
#[test]
fn test418() {
    let v = { //or v: i32
        let mut x = 1;
        x += 2 //;
        //x
    };
    assert_eq!(v, ()); //assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test419() {
    let v = {
        let x = 3;
        x
    };
    assert_eq!(v, 3);
    println!("Success!");
}

#[test]
fn test420() {
    let s = sum(1 , 2);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum(x: i32, y: i32) -> i32 {
    x + y //not ";"
}
/// https://practice.course.rs/basic-types/functions.html
#[test]
fn test421() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum1(x, y);
    assert_eq!(s, 3);
    println!("Success!");
}
fn sum1(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test422() {
    print();
}
// Replace i32 with another type
fn print() -> () {
    println!("Success!")
}

#[test]
// Solve it in two ways
// DON'T let `println!` work
fn test423() {
    never_return();
    println!("Failed!");
}
fn never_return() -> () { //було так fn never_return() -> !
    // Implement this function, don't modify the fn signatures

}

#[test]
fn test424() {
    println!("Success!");
}
fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    // Rather than returning a None, we use a diverging function instead
    never_return_fn()
}
// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    todo!() //or panic!() or unimplemented!()
}

#[test]
fn test425() {
    // FILL in the blank
    let b = false;
    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };
    println!("Exercise Failed if printing out this line!");
}