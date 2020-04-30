#[test]
fn iterator() {
    // original
    let v0 = vec![1, 4, 5, 9, 0];
    // clone
    let mut v1 = v0.clone();
    for i in &v1 {
        println!("v1's item is {}", i);
    }

    // each item +2 using mut iterator
    let v1_iter = v1.iter_mut();
    for i in v1_iter {
        *i += 2;
        println!("item is {}", i);
    }
    println!("{:?}", v1);

    // check the result
    for i in 0..v0.len() {
        assert_eq!(v0[i] + 2, v1[i]);
    }

    let mut v1_iter2 = v1.iter();
    assert_eq!(v1_iter2.next(), Some(&3));
}
