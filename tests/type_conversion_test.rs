#[test]
fn basic() {
    let guess = String::from("-34");
    let guess: i32 = guess.parse().expect("unable to parse:{}");
    println!("the guess is {}", guess);
}
