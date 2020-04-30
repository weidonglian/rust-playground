mod m1 {
    fn f1(name: &str) {
        println!("calling f1 with:{}", name);
    }

    pub mod m2 {
        use super::f1;
        pub fn f2(count: isize) {
            print!("calling f2 with:{}", count);
            f1("lianweidong");
        }
    }
}

#[test]
fn basic() {
    use m1::m2::f2 as func2;
    func2(10);
}
