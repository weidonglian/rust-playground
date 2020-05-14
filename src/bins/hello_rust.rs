fn main() {
    println!("Hello, world!");

    let _s = "weidonglian".to_string();
    //s.fuck();

    let c = |num: i32| -> i32 {
        println!("num is {}", num);
        num + 2
    };
    println!("result is {}", c(2));

    let d = |num| num + 2;
    println!("result is {}", d(2));
}
