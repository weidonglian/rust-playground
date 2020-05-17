#[allow(dead_code)]
fn give_princess(gift: &str) {
    // Princesses hate snakes, so we need to stop if she disapproves!
    if gift == "snake" {
        panic!(format!("{} with {}", "AAAaaaaa!!!!", gift));
    }

    println!("I love {}s!!!!!", gift);
}

#[test]
#[should_panic]
fn basic() {
    give_princess("teddy bear");
    give_princess("snake");
}

#[test]
fn iter_over_results() {
    let strings = vec!["tofu", "93", "18"];
    let numbers = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<_>>();
    assert_eq!(&numbers, &[93, 18]);
    println!("Results: {:?}", numbers);
}

#[test]
fn iter_over_results_fail_all() {
    let strings = vec!["93", "tofu", "18"];
    let numbers: Result<Vec<_>, _> = strings.into_iter().map(|s| s.parse::<i32>()).collect();
    assert!(numbers.is_err());
}

#[test]
fn result_type() {
    #[derive(Debug)]
    enum MyError {
        A,
        B,
    };

    let e1: Result<i32, MyError> = Err(MyError::A);
    let e2 = Err::<i32, _>(MyError::B);
    println!("err is {:?} and {:?}", e1, e2);
}
