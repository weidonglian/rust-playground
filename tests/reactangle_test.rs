use rust_playground::rectangle::Rectangle;

#[test]
fn can_hold() {
    let a = Rectangle::new(24, 50);
    let b = Rectangle::new(10, 20);
    assert!(a.can_hold(&b));
}
