use std::fmt;
use std::thread;
use std::time::Duration;

fn pool_print<T: Send + Sync + std::fmt::Display + 'static>(val: T) {
    thread::spawn(move || println!("{}", val));
}

#[test]
fn basic_demo() {
    pool_print("hello"); // &'static str
    pool_print(String::from("hello")); // String
    pool_print(5); // i32
    struct Foo {
        string: String,
        v: Vec<f64>,
    };
    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {:?}", self.string, self.v)
        }
    }
    // Arbitrary struct containing String and Vec<f64>
    pool_print(Foo {
        string: String::from("hi"),
        v: vec![1.2, 2.3],
    });
    thread::sleep(Duration::new(1, 0));
}
