use tokio::runtime::Runtime;

async fn hello(name: &str) {
    println!("hello, {}", name);
}

#[test]
fn block_on_demo() {
    let future = hello("Jimmy");
    let rt = Runtime::new().unwrap();
    println!("before...");
    rt.block_on(future);
    println!("after...");
}
