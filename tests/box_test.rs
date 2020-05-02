use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        return MyBox(x);
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

#[test]
fn basic() {
    let x = 5;
    let mut y = Box::new(x);
    assert_eq!(x, *y);

    *y = 4;
    assert_eq!(*y, 4);
    assert_eq!(x, 5);
}

#[test]
fn mybox_basic() {
    let x = 5;
    let mut y = MyBox::new(x);
    assert_eq!(x, *y);

    *y = 4;
    assert_eq!(*y, 4);
    assert_eq!(x, 5);
}
