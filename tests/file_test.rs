use std::fs::File;
use std::io::Read;
use std::path::Path;

#[test]
#[should_panic]
fn basic() {
    // create path
    let path = Path::new("foo.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Err(why) => panic!("could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // read the file
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("fail to read to string");
    println!("the file:{} content is:\n{}", display, buf);
}

#[test]
fn display_path() {
    let path = Path::new(".");
    println!("the current path is:{}", path.display());

    let new_path = path.join("a").join("b");
    println!(
        "the new path is:{}",
        new_path.to_str().expect("unknown path")
    );
}
