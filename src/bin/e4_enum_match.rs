enum CollectionType {
    Playlist,
    Soundtrack,
    Schedule,
}

fn collection_type_name(collection_type: CollectionType) -> String {
    match collection_type {
        CollectionType::Playlist => String::from("playlist"),
        CollectionType::Schedule => String::from("schedule"),
        CollectionType::Soundtrack => String::from("soundtrack"),
    }
}

fn main() {
    println!(
        "My collection type is {}",
        collection_type_name(CollectionType::Playlist)
    )
}
