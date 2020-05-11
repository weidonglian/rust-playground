#[test]
fn bastic() {
    let mut s = String::from("weidonglian");
    s.push_str("xiaogai");
    println!("the string is {}", s);
}

#[test]
fn conversion() {
    let buffer = [0; 4];
    let content = String::from_utf8_lossy(&buffer);
    assert_eq!(String::from("b\0\0\0\0e"), format!("b{}e", content));
}
