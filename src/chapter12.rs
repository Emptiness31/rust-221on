///https://practice.course.rs/type-conversions/as.html
#[test]
fn test121() {
    // FIX the errors and FILL in the blank
    // DON'T remove any code
    let decimal = 97.123_f32;
    let integer: u8 = decimal as u8;
    let _c1: char = decimal as u8 as char;
    let _c2 = integer as char;
    assert_eq!(integer + 1, 'b' as u8);
    println!("Success!");
}

#[allow(overflowing_literals)]
#[test]
fn test122() {
    assert_eq!(u8::MAX, 255);
    // The max of `u8` is 255 as shown above.
    // so the below code will cause an overflow error: literal out of range for `u8`.
    // PLEASE looking for clues within compile errors to FIX it.
    // DON'T modify any code in main.
    let _v = (1000 % 256) as u8; //було let _v = 1000 as u8;
    println!("Success!");
}
#[allow(overflowing_literals)]
#[test]
fn test123() {
    assert_eq!(1000u16, 1000);
    assert_eq!((1000 % 256) as u8, 232); //було так assert_eq!(1000 as u8, 232);
    // For positive numbers, this is the same as the modulus
    println!("1000 mod 256 is : {}", 1000 % 256);
    assert_eq!(-1_i8 as u8, 255);
    // Since Rust 1.45, the `as` keyword performs a *saturating cast*
    // when casting from float to int. If the floating point value exceeds
    // the upper bound or is less than the lower bound, the returned value
    // will be equal to the bound crossed.
    assert_eq!(300.1_f32 as u8, 255);
    assert_eq!(-100.1_f32 as u8, 0);
    // This behavior incurs a small runtime cost and can be avoided
    // with unsafe methods, however the results might overflow and
    // return **unsound values**. Use these methods wisely:
    unsafe {
        // 300.0 is 44
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        // nan as u8 is 0
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}

#[test]
fn test124() {
    // FILL in the blanks
    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address: usize = p1 as usize;
    let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
    let p2: *mut i32 = second_address as *mut i32; // p2 points to the 2nd element in values
    unsafe {
        // Add one to the second element
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
    println!("Success!");
}

#[test]
fn test125() {
    let arr :[u64; 13] = [0; 13];
    assert_eq!(size_of_val(&arr), 8 * 13);
    let a: *const [u64] = &arr;
    let b = a as *const [u8];
    unsafe {
        assert_eq!(size_of_val(&*b), 13)
    }
    println!("Success!");
}

///https://practice.course.rs/type-conversions/from-into.html
#[test]
fn test126() {
    // impl From<bool> for i32
    let i1: i32 = false.into();
    let i2: i32 = i32::from(false);
    assert_eq!(i1, i2);
    assert_eq!(i1, 0);
    // FIX the error in two ways
    /* 1. use a similar type which `impl From<char>`, maybe you
    should check the docs mentioned above to find the answer */
    // 2. a keyword from the last chapter
    let _i3: u32 = 'a'.into();
    // FIX the error in two ways
    let _s: String = 'a'.into();
    println!("Success!");
}

#[test]
fn test127() {
    // From is now included in `std::prelude`, so there is no need to introduce it into the current scope
    // use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        // IMPLEMENT `from` method
        fn from(n: i32) -> Number {
            Number { value: n }
        }
    }
    // FILL in the blanks
    let num = Number::from(30);
    assert_eq!(num.value, 30);
    let num: Number = 30_i32.into();
    assert_eq!(num.value, 30);
    println!("Success!");
}

#[test]
fn test128() {
    use std::fs;
    use std::io;
    use std::num;
    enum CliError {
        IoError(io::Error),
        ParseError(num::ParseIntError),
    }
    impl From<io::Error> for CliError {
        // IMPLEMENT from method
        fn from(e: io::Error) -> CliError {
            CliError::IoError(e)
        }
    }
    impl From<num::ParseIntError> for CliError {
        // IMPLEMENT from method
        fn from(e: num::ParseIntError) -> CliError {
            CliError::ParseError(e)
        }
    }
    fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
        // ? automatically converts io::Error to CliError
        let contents = fs::read_to_string(&file_name)?;
        // num::ParseIntError -> CliError
        let num: i32 = contents.trim().parse()?;
        Ok(num)
    }
    println!("Success!");
}

#[test]
fn test129() {
    // TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
    // use std::convert::TryInto;
    let n: i16 = 256;
    // Into trait has a method `into`,
    // hence TryInto has a method ?
    let n: u8 = match n.try_into() {
        Ok(n) => n,
        Err(e) => {
            println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            0
        }
    };
    assert_eq!(n, 0);
    println!("Success!");
}

#[test]
fn test1210() {
    #[derive(Debug, PartialEq)]
    struct EvenNum(i32);
    impl TryFrom<i32> for EvenNum {
        type Error = ();
        // IMPLEMENT `try_from`
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNum(value))
            } else {
                Err(())
            }
        }
    }
    assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
    assert_eq!(EvenNum::try_from(5), Err(()));
    // FILL in the blanks
    let result: Result<EvenNum, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNum(8)));
    let result: Result<EvenNum, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
    println!("Success!");
}

///https://practice.course.rs/type-conversions/others.html
#[test]
fn test1211() {
    use std::fmt;
    struct Point {
        x: i32,
        y: i32,
    }
    impl fmt::Display for Point {
        // IMPLEMENT fmt method
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The point is ({}, {})", self.x, self.y)
        }
    }
    let origin = Point { x: 0, y: 0 };
    // FILL in the blanks
    assert_eq!(origin.to_string(), "The point is (0, 0)");
    assert_eq!(format!("{}", origin), "The point is (0, 0)");
    println!("Success!");
}

#[test]
fn test1212() {
    // To use `from_str` method, you need to introduce this trait into the current scope.
    use std::str::FromStr;
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let from_str = i32::from_str("20").unwrap();
    let sum = parsed + turbo_parsed + from_str;
    assert_eq!(sum, 35);
    println!("Success!");
}

#[test]
fn test1213() {
    use std::str::FromStr;
    use std::num::ParseIntError;
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32
    }
    impl FromStr for Point {
        type Err = ParseIntError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                .split(',')
                .map(|x| x.trim())
                .collect();
            let x_fromstr = coords[0].parse::<i32>()?;
            let y_fromstr = coords[1].parse::<i32>()?;
            Ok(Point { x: x_fromstr, y: y_fromstr })
        }
    }
    // FILL in the blanks in two ways
    // DON'T change code anywhere else
    let p = Point::from_str("(3,4");
    assert_eq!(p.unwrap(), Point{ x: 3, y: 4} );
    println!("Success!");
}