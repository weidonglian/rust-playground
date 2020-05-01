#[test]
fn basic_demo() {
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

#[test]
fn iterator_next() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 4, 8];
    let total: i32 = v1.iter().sum();
    assert_eq!(total, 1 + 2 + 4 + 8);
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 4, 8];
    let v2: Vec<_> = v1.iter().map(|x| x + 2).collect();
    assert_eq!(v2, vec![3, 4, 6, 10]);
}

#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    // iter() returns an iterator of slices.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn into_iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.into_iter();

    // into_iter() returns an iterator from a value.
    assert_eq!(v1_iter.next(), Some(1));
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_mut_demo() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();

    // iter_mut() returns an iterator that allows modifying each value.
    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);
}
