use enum_iterator::IntoEnumIterator;

#[derive(IntoEnumIterator)]
enum CollectionType {
    Playlist,
    Soundtrack,
    Schedule,
}

enum ViolationId {
    Dup(String),
    DMCA(i32),
    Albume(String, String),
    Artist([i32; 2]),
    Track { a: String, b: i16 },
}

fn collection_type_name(collection_type: CollectionType) -> String {
    match collection_type {
        CollectionType::Playlist => format!("playlist"),
        CollectionType::Schedule => format!("schedule"),
        CollectionType::Soundtrack => format!("soundtrack"),
    }
}

#[test]
fn basic() {
    assert_eq!(CollectionType::VARIANT_COUNT, 3);
    println!(
        "My collection type is {}",
        collection_type_name(CollectionType::Playlist)
    );

    let violations = vec![
        ViolationId::Dup(String::from("my dup track")),
        ViolationId::DMCA(23),
        ViolationId::Albume(String::from("a"), String::from("b")),
        ViolationId::Artist([23, 34]),
        ViolationId::Track {
            a: String::from("aa"),
            b: 23,
        },
    ];

    for i in &violations {
        match i {
            ViolationId::Dup(val) => println!("Dup - violation:{}", val),
            ViolationId::DMCA(val) => println!("DMCA - violation:{}", val),
            ViolationId::Albume(a, b) => println!("Album - violation: a:{}, b:{}", a, b),
            ViolationId::Artist(val) => println!("Artist - violation: {}", val.len()),
            ViolationId::Track { a, b } => println!("Track - violation: {}, {}", a, b),
        }
    }
}

#[test]
fn while_let() {
    let mut stack = Vec::new();
    stack.push(10);
    stack.push(20);
    stack.push(56);

    for (idx, val) in stack.iter().enumerate() {
        println!("the value at index:{} is {}", idx, val);
    }

    let mut stack_iter = stack.iter();
    if let Some(first) = stack_iter.next() {
        println!("the first of iterator is {}", first);
    }
    for (idx, val) in stack_iter.enumerate() {
        // idx 0 from second element
        println!("the partial iterator: {} for {}", idx, val);
    }

    while let Some(val) = stack.pop() {
        println!("the pop value is {}", val);
    }
}

#[test]
fn let_pattern() {
    let point = (10, 20); // tuple
    let (x, y) = point;
    assert_eq!(x, 10);
    assert_eq!(y, 20);

    let &(xx, yy) = &point;
    assert_eq!(xx, 10);
    assert_eq!(yy, 20);

    let pair = (String::from("KeyJim"), String::from("ValGreen"));
    let (key, val) = pair;
    assert_eq!(key, "KeyJim");
    assert_eq!(val, "ValGreen");
    // println!("the pair is {:?}", pair); can not use a moved value

    struct Point {
        x: i32,
        y: i32,
    };

    let p = Point { x: 0, y: 1 };
    let Point { x, y } = p;
    assert_eq!(x, p.x);
    assert_eq!(y, p.y);
}

#[test]
fn match_range() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
