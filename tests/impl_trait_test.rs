
trait Trait {
    fn format(&self) -> String;
}

impl Trait for i32 {
    fn format(&self) -> String {
        return self.to_string()
    }
}

// before `impl trait` feature, we have to do like below.
// kinda similar to `std::shared_ptr<Trait>` in C++.
fn return_a_trait_object() ->Box<dyn Trait> {
    Box::new(5)
}

// Now do not need dynamic dispatch
fn return_a_trait_impl() -> impl Trait {
    5
}

#[test]
fn check_return_trait() {
    let x= return_a_trait_impl();
    assert_eq!(x.format(), 5.to_string());

    let y = return_a_trait_object();
    assert_eq!(y.format(), 5.to_string());
}