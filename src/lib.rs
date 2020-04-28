pub struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    pub fn new(width: u32, height: u32) -> Reactangle {
        return Reactangle {
            width: width,
            height: height,
        };
    }

    pub fn can_hold(&self, other: &Reactangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reactangle_can_hold() {
        let a = Reactangle::new(24, 50);
        let b = Reactangle::new(10, 20);
        assert!(a.can_hold(&b));
    }
}
