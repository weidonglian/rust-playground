#[test]
fn basic() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get_mut(2) {
        Some(third) => {
            println!("The third element is {} before", third);
            *third = 20;
        }
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("the element is:{}", i);
    }

    // in rust the type is determined by the expression,
    // which is different from c++ auto and auto&, auto*.
    let mut x = 1;
    println!("x is {} before", x);
    let y = &mut x;
    *y = 5;
    println!("x is {} after", x);

    // an useful but confusing feature for c++ developer
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // normal deduction same as let
        *i += 50;
    }

    for i in &v {
        // i is a &i32
        println!("element is {}", i);
    }

    // destructive operator
    for &i in &v {
        // i is a i32
        println!("element is {}", i);
    }

    struct Foo(isize); // newtype with isize similar to Golang's feature
    let tv = vec![Foo(100), Foo(32), Foo(57)];
    for Foo(t) in &tv {
        // let destruct assignment
        print!("ele is {}", t);
    }
    for i in &tv {
        // let destruct assignment
        print!("ele is {}", i.0); // .0 is access the first element
    }
}

#[test]
fn by_value() {
    let v = vec![10, 20, 50, 45];
    for i in v {
        println!("the value i is {}", i);
    }
    // println!("the v is {:?}", v); can not use it since the loop above move it.
}

#[test]
fn by_ref() {
    let mut v = vec![10, 20, 50, 45];
    let o = v.clone();
    for &i in &v {
        println!("the value is {}", i)
    }
    assert_eq!(v, o);

    for i in &mut v {
        *i = 30;
    }
    assert_ne!(v, o);
}
