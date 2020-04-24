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

fn main() {
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
