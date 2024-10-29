/// https://practice.course.rs/generics-traits/generics.html
#[test]
fn test101() {
    // Fill in the blanks to make it work
    struct A;          // Concrete type `A`.
    struct S(A);       // Concrete type `S`.
    struct SGen<T>(T); // Generic type `SGen`.
    fn reg_fn(_s: S) {}
    fn gen_spec_t(_s: SGen<A>) {}
    fn gen_spec_i32(_s: SGen<i32>) {}
    fn generic<T>(_s: SGen<T>) {}
    // Using the non-generic functions
    reg_fn(S(A)); // Concrete type.
    gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
    gen_spec_i32(SGen(9)); // Implicitly specified type parameter `i32`.
    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(SGen('L'));
    // Implicitly specified type parameter `char` to `generic()`.
    generic(SGen('W'));
    println!("Success!");
}

#[test]
fn test102() {
    // Implement the generic function below.
    fn sum<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a + b
    }
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
    println!("Success!");
}

#[test]
fn test103() {
    // Implement struct Point to make it work.
    struct Point<T> {
        x: T,
        y: T,
    }
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    println!("Success!");
}

#[test]
fn test104() {
    // Modify this struct to make the code work
    struct Point<T> {
        x: T,
        y: String,
    }
    // DON'T modify this code.
    let _p = Point{x: 5, y : "hello".to_string()};
    println!("Success!");
}

#[test]
fn test105() {
    // Add generic for Val to make the code work, DON'T modify the code in `main`.
    struct Val<T> {
        val: T,
    }
    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}

#[test]
fn test106() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: '中'};
    let p3 = p1.mixup(p2);
    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, '中');
    println!("Success!");
}

#[test]
fn test107() {
    // Fix the errors to make the code work.
    struct Point<T> {
        x: T,
        y: T,
    }
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    let p: Point<f64> = Point{x: 5.0, y: 10.0};
    println!("{}",p.distance_from_origin());
}
/// https://practice.course.rs/generics-traits/const-generics.html
#[test]
fn test108() {
    struct Array<T, const N: usize> {
        data : [T; N]
    }
    let _arrays = [
        Array{
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3],
        },
        Array {
            data: [1, 2, 3]
        }
    ];
    println!("Success!");
}

#[test]
fn test109() {
    // Fill in the blanks to make it work.
    fn print_array<T, const N: usize>(arr: [T; N])
    where
        T: std::fmt::Debug,
    {
        println!("{:?}", arr);
    }
    let arr = [1, 2, 3];
    print_array(arr);

    let arr = ["hello", "world"];
    print_array(arr);
}

#[test]
fn test1010() {
    // Fix the errors in main.
    #![allow(incomplete_features)]
    #![feature(generic_const_exprs)]
    fn check_size<T>(_val: T)
    where
        // Assert<{ size_of::<T>() < 768 }>: IsTrue,
    {
        //...
    }
    check_size([0u8; 767]);
    check_size([0i32; 191]);
    check_size(["hello你好"; 11]); // Size of &str ?
    check_size([(); 0].map(|_| "hello你好".to_string()));  // Size of String?
    check_size(['中'; 3]); // Size of char ?
    println!("Success!");
}
pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
/// https://practice.course.rs/generics-traits/traits.html
#[test]
fn test1011() {
    // Fill in the two impl blocks to make the code work.
    // DON'T modify the code in `main`.
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }
        fn say_something(&self) -> String;
    }
    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }
        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }
    let s = Student {};
    assert_eq!(s.say_hi(), "hi");
    assert_eq!(s.say_something(), "I'm a good student");
    let t = Teacher {};
    assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
    assert_eq!(t.say_something(), "I'm not a bad teacher");
    println!("Success!");
}

#[test]
fn test1012() {
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);
    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);
    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;
            Centimeters(inches as f64 * 2.54)
        }
    }
    // ADD some attributes to make the code work!
    // DON'T modify other code!
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);
    let _one_second = Seconds(1);
    println!("One second looks like: {:?}", _one_second);
    let _this_is_true = _one_second == _one_second;
    let _this_is_false = _one_second > _one_second;
    let foot = Inches(12);
    println!("One foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} than one meter.", cmp);
}

#[test]
fn test1013() {
    // Implement fn multiply to make the code work.
    // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
    // So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
    fn multiply<T>(a: T, b: T) -> T
    where
        T: std::ops::Mul<Output = T>,
    {
        a * b
    }
    assert_eq!(6, multiply(2u8, 3u8));
    assert_eq!(5.0, multiply(1.0, 5.0));
    println!("Success!");
}

#[test]
fn test1014() {
    // Fix the errors, DON'T modify the code in `main`.
    use std::ops;
    struct Foo;
    struct Bar;
    #[derive(Debug, PartialEq)]
    struct FooBar;
    #[derive(Debug, PartialEq)]
    struct BarFoo;
    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;
        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }
    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;
        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.
    assert_eq!(Foo + Bar, FooBar);
    assert_eq!(Foo - Bar, BarFoo);
    println!("Success!");
}

#[test]
fn test1015() {
    // Implement `fn summary` to make the code work.
    // Fix the errors without removing any code line
    trait Summary {
        fn summarize(&self) -> String;
    }
    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }
    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }
    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }
    let post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "Weibo seems to be worse than Tweet".to_string(),
    };
    summary(&post);
    summary(&weibo);
    println!("{:?}", post);
    println!("{:?}", weibo);
    // Implement `fn summary` below.
    fn summary(t: &impl Summary) {
        println!("{}", t.summarize());
    }
}

#[test]
fn test1016() {
    struct Sheep {}
    struct Cow {}
    trait Animal {
        fn noise(&self) -> String;
    }
    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }
    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }
    // Returns some struct that implements Animal, but we don't know which one at compile time.
    // FIX the errors here, you can make a fake random, or you can use trait object.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}

#[test]
fn test1017() {
    assert_eq!(sum(1, 2), 3);
}
// Implement `fn sum` with trait bound in two ways.
fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    x + y
}

// FIX the errors.
struct Pair<T> {
    x: T,
    y: T,
}
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: std::fmt::Debug + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {:?}", self.x);
        } else {
            println!("The largest member is y = {:?}", self.y);
        }
    }
}
#[derive(Debug, PartialEq, PartialOrd)]
struct Unit(i32);
#[test]
fn test1018() {
    let pair = Pair{
        x: Unit(1),
        y: Unit(3)
    };
    pair.cmp_display();
}

#[test]
fn test1019() {
    // Fill in the blanks to make it work
    fn example1() {
        // `T: Trait` is the commonly used way.
        // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
        struct Cacher<T: Fn(u32) -> u32> {
            calculation: T,
            value: Option<u32>,
        }
        impl<T: Fn(u32) -> u32> Cacher<T> {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(10), 11);
        assert_eq!(cacher.value(15), 11);
    }
    fn example2() {
        // We can also use `where` to construct `T`
        struct Cacher<T>
        where T: Fn(u32) -> u32,
        {
            calculation: T,
            value: Option<u32>,
        }
        impl<T> Cacher<T>
        where T: Fn(u32) -> u32,
        {
            fn new(calculation: T) -> Cacher<T> {
                Cacher {
                    calculation,
                    value: None,
                }
            }
            fn value(&mut self, arg: u32) -> u32 {
                match self.value {
                    Some(v) => v,
                    None => {
                        let v = (self.calculation)(arg);
                        self.value = Some(v);
                        v
                    }
                }
            }
        }
        let mut cacher = Cacher::new(|x| x+1);
        assert_eq!(cacher.value(20), 21);
        assert_eq!(cacher.value(25), 21);
    }
    example1();
    example2();
    println!("Success!");
}
/// https://practice.course.rs/generics-traits/trait-object.html
#[test]
fn test1020() {
    // FILL in the blank.
    trait Bird {
        fn quack(&self) -> String;
    }
    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }
    impl Bird for Duck {
        fn quack(&self) -> String{
            "duck duck".to_string()
        }
    }
    impl Bird for Swan {
        fn quack(&self) -> String{
            "swan swan".to_string()
        }
    }
    let duck = Duck;
    duck.swim();
    let bird = hatch_a_bird(2);
    // This bird has forgotten how to swim, so below line will cause an error.
    // bird.swim();
    // But it can quak.
    assert_eq!(bird.quack(), "duck duck");
    let bird = hatch_a_bird(1);
    // This bird has forgotten how to fly, so below line will cause an error.
    // bird.fly();
    // But it can quak too.
    assert_eq!(bird.quack(), "swan swan");
    println!("Success!");
    // IMPLEMENT this function.
    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        match species {
            1 => Box::new(Swan),
            2 => Box::new(Duck),
            _ => panic!(),
        }
    }
}

#[test]
fn test1021() {
    // FILL in the blank to make the code work.
    trait Bird {
        fn quack(&self);
    }
    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }
    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }
    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }
    let birds: [&dyn Bird; 2] = [&Duck, &Swan];
    for bird in birds {
        bird.quack();
        // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
        // So, the code below will cause an error.
        // bird.fly();
    }
}

#[test]
fn test1022() {
    // FILL in the blanks.
    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    let x = 1.1f64;
    let y = 8u8;
    // Draw x.
    draw_with_box(Box::new(x));
    // Draw y.
    draw_with_ref(&y);
    println!("Success!");
    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }
    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
}

#[test]
fn test1023() {
    trait Foo {
        fn method(&self) -> String;
    }
    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }
    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // IMPLEMENT below with generics.
    fn static_dispatch<T: Foo>(a: T) {
        // or: fn static_dispatch(a: impl Foo)
        a.method();
    }
    // Implement below with trait objects.
    fn dynamic_dispatch(a: &dyn Foo) {
        a.method();
    }
    let x = 5u8;
    let y = "Hello".to_string();
    static_dispatch(x);
    dynamic_dispatch(&y);
    println!("Success!");
}

#[test]
fn test1024() {
    // Use at least two approaches to make it work.
    // DON'T add/remove any code line.
    trait MyTrait {
        fn f(&self) -> Self;
    }
    impl MyTrait for u32 {
        fn f(&self) -> Self { 42 }
    }
    impl MyTrait for String {
        fn f(&self) -> Self { self.clone() }
    }
    fn my_function<T: MyTrait>(x: T) -> T {
        x.f()
    }
    my_function(13_u32);
    my_function(String::from("abc"));
    println!("Success!");
}
/// https://practice.course.rs/generics-traits/advanced-traits.html
#[test]
fn test1025() {
    struct Container(i32, i32);
    // USING associated types to re-implement trait Contains.
    trait Contains {
        type A;
        type B;
        fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
        fn first(&self) -> i32;
        fn last(&self) -> i32;
    }
    impl Contains for Container {
        type A = i32;
        type B = i32;
        fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
            (&self.0 == number_1) && (&self.1 == number_2)
        }
        // Grab the first number.
        fn first(&self) -> i32 { self.0 }

        // Grab the last number.
        fn last(&self) -> i32 { self.1 }
    }

    fn difference<C: Contains>(container: &C) -> i32 {
        container.last() - container.first()
    }
    let number_1 = 3;
    let number_2 = 10;
    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}",
             &number_1, &number_2,
             container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}

#[test]
fn test1026() {
    use std::ops::Sub;
    #[derive(Debug, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }
    // FILL in the blank in three ways: two of them use the default generic  parameters, the other one not.
    // Notice that the implementation uses the associated type `Output`.
    impl<T: Sub<Output = T>> Sub for Point<T> {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Point {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }
    assert_eq!(Point { x: 2, y: 3 } - Point { x: 1, y: 0 },
               Point { x: 1, y: 3 });
    println!("Success!");
}

#[test]
fn test1027() {
    trait Pilot {
        fn fly(&self) -> String;
    }
    trait Wizard {
        fn fly(&self) -> String;
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) -> String {
            String::from("This is your captain speaking.")
        }
    }
    impl Wizard for Human {
        fn fly(&self) -> String {
            String::from("Up!")
        }
    }
    impl Human {
        fn fly(&self) -> String {
            String::from("*waving arms furiously*")
        }
    }
    let person = Human;
    assert_eq!(Pilot::fly(&person), "This is your captain speaking.");
    assert_eq!(Wizard::fly(&person), "Up!");
    assert_eq!(person.fly(), "*waving arms furiously*");
    println!("Success!");
}

#[test]
fn test1028() {
    trait Person {
        fn name(&self) -> String;
    }
    // Person is a supertrait of Student.
    // Implementing Student requires you to also impl Person.
    trait Student: Person {
        fn university(&self) -> String;
    }
    trait Programmer {
        fn fav_language(&self) -> String;
    }
    // CompSciStudent (computer science student) is a subtrait of both Programmer
    // and Student. Implementing CompSciStudent requires you to impl both supertraits.
    trait CompSciStudent: Programmer + Student {
        fn git_username(&self) -> String;
    }
    fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
        format!(
            "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
            student.name(),
            student.university(),
            student.fav_language(),
            student.git_username()
        )
    }
    struct CSStudent {
        name: String,
        university: String,
        fav_language: String,
        git_username: String,
    }
    // IMPLEMENT the necessary traits for CSStudent to make the code work
    impl Person for CSStudent {
        fn name(&self) -> String {
            self.name.clone()
        }
    }
    impl Student for CSStudent {
        fn university(&self) -> String {
            self.university.clone()
        }
    }
    impl Programmer for CSStudent {
        fn fav_language(&self) -> String {
            self.fav_language.clone()
        }
    }
    impl CompSciStudent for CSStudent {
        fn git_username(&self) -> String {
            self.git_username.clone()
        }
    }
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "XXX".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };
    // FILL in the blank
    println!("{}", comp_sci_student_greeting(&student));
}

#[test]
fn test1029() {
    use std::fmt;
    // DEFINE a newtype `Pretty` here
    struct Pretty(String);
    impl fmt::Display for Pretty {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "\"{}\"", self.0.clone() + ", world")
        }
    }
    let w = Pretty("hello".to_string());
    println!("w = {}", w);
}