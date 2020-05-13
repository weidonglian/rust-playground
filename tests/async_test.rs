use futures::executor::block_on;

async fn hello(name: &str) {
    println!("hello, {}", name);
}

#[test]
fn block_on_demo() {
    let future = hello("Jimmy");
    println!("before...");
    block_on(future);
    println!("after...");
}
