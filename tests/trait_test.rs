use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

#[test]
fn from_trait() {
    let num = Number::from(45);
    println!("num is {:?}", num);
}

#[test]
fn into_trait() {
    let num: Number = 45.into();
    println!("num is {:?}", num);
}
