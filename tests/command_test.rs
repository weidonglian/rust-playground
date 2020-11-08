#[test]
fn basic() {
    use std::process::Command;
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap_or_else(|e| {
            panic!("failed with {}", e);
        });
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);
        println!("success with:{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);
        println!("failed with:{}", s)
    }
}
