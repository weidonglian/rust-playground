use rust_playground::Reactangle;

#[test]
fn can_hold() {
    let a = Reactangle::new(24, 50);
    let b = Reactangle::new(10, 20);
    assert!(a.can_hold(&b));
}
