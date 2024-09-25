#[test]
fn test1() {
    let x: i32 = 5;
    let mut _y = 5;
    _y = x;
    let _z: i32 = 10; // Type of z ?
    println!("Success!");
}

#[test]
fn test2() {
    let _v: u16 = 38_u8 as u16;
    println!("Success!");
}

#[test]
fn test3() {
    let x: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&x));
    println!("Success!");
}
// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);
    println!("Success!");
}