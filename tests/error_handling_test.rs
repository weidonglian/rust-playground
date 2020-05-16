#[allow(dead_code)]
fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" {
        panic!(format!("{} with {}", "AAAaaaaa!!!!", gift));
    }

    println!("I love {}s!!!!!", gift);
}

#[test]
//#[should_panic]
fn basic() {
    give_princess("teddy bear");
    give_princess("snake");
}
