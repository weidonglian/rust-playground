#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Rectangle {
        return Rectangle {
            width,
            height,
        };
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rectangle_can_hold() {
        let a = Rectangle::new(24, 50);
        let b = Rectangle::new(10, 20);
        assert!(a.can_hold(&b));
    }
}
