use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("The secret number is:{}", secret_number);

        println!("Please input your guess:");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("fail to read a line.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guess number is:{}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("too small!"),
            std::cmp::Ordering::Greater => println!("too big!"),
            std::cmp::Ordering::Equal => {
                println!("you win!!");
                break;
            }
        }
    }
}
