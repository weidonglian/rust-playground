use std::collections::HashMap;

#[test]
fn basic() {
    let mut scores = HashMap::new();
    scores.insert(String::from("hello"), 10);
    scores.insert(String::from("lian"), 20);
    for (k, v) in &scores {
        println!("key,value=[{}, {}]", k, v);
    }
    println!("{:#?}", scores);
}
