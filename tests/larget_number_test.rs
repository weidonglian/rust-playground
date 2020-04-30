use std::cmp::PartialOrd;

fn find_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest_item = list[0];
    for &item in list {
        if item > largest_item {
            largest_item = item;
        }
    }
    return largest_item;
}

#[test]
fn basic() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(result, 100);

    let result = find_largest(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(result, 'y');

    let result = find_largest(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}
