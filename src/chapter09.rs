/// https://practice.course.rs/method.html
#[test]
fn test91() {
    struct Rectangle {
        width: u32,
        height: u32,
    }
    impl Rectangle {
        // Complete the area method which return the area of a Rectangle.
        fn area (self) -> u32 {
            self.width * self.height
        }
    }
    let rect1 = Rectangle { width: 30, height: 50 };
    assert_eq!(rect1.area(), 1500);
    println!("Success!");
}

#[test]
fn test92() {
    // Only fill in the blanks, DON'T remove any line!
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }
    impl TrafficLight {
        pub fn show_state(&self)  {
            println!("the current state is {}", self.color);
        }
    }
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    // Don't take the ownership of `light` here.
    light.show_state();
    // ... Otherwise, there will be an error below
    println!("{:?}", light);
}

#[test]
fn test93() {
    struct TrafficLight {
        color: String,
    }
    impl TrafficLight {
        // Using `Self` to fill in the blank.
        pub fn show_state(self: &Self)  {
            println!("the current state is {}", self.color);
        }
        // Fill in the blank, DON'T use any variants of `Self`.
        pub fn change_state(mut self) {
            self.color = "green".to_string()
        }
    }
    println!("Success!");
}

#[test]
fn test94() {
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }
    impl TrafficLight {
        // 1. Implement an associated function `new`,
        // 2. It will return a TrafficLight contains color "red"
        // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
        pub fn new() -> Self {
            Self {
                color: String::from("red"),
            }
        }

        pub fn get_state(&self) -> &str {
            &self.color
        }
    }
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");
    println!("Success!");
}

#[test]
fn test95() {
    struct Rectangle {
        width: u32,
        height: u32,
    }
    // Using multiple `impl` blocks to rewrite the code below.
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    impl Rectangle {
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    println!("Success!");
}

#[test]
fn test96() {
    #[derive(Debug)]
    enum TrafficLightColor {
        Red,
        Yellow,
        Green,
    }
    // Implement TrafficLightColor with a method.
    impl TrafficLightColor {
        fn color(&self) -> &'static str {
            match *self {
                Self::Red => "red",
                Self::Yellow => "yellow",
                Self::Green => "green",
            }
        }
    }
    let c = TrafficLightColor::Yellow;
    assert_eq!(c.color(), "yellow");
    println!("{:?}",c);
}