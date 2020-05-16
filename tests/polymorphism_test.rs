trait Playable {
    fn play(&self);
}

struct JazzSong {
    name: String,
}

struct ClassicSong {
    category: String,
}

impl Playable for JazzSong {
    fn play(&self) {
        println!("playing jazz:{} music now...", self.name);
    }
}

impl Playable for ClassicSong {
    fn play(&self) {
        println!("playing classic:{} music now...", self.category);
    }
}

#[test]
fn poly_by_reference() {
    let jazz1 = JazzSong {
        name: String::from("ABC"),
    };
    let jazz2 = JazzSong {
        name: String::from("CDF"),
    };
    let classic1 = ClassicSong {
        category: String::from("UUUU"),
    };
    let classic2 = ClassicSong {
        category: String::from("PPPPP"),
    };
    let songs: Vec<&dyn Playable> = vec![&jazz1, &classic2, &jazz2, &classic1];
    for song in songs {
        song.play();
    }
}

#[test]
fn poly_by_box() {
    let jazz1 = Box::new(JazzSong {
        name: String::from("ABC"),
    });
    let jazz2 = Box::new(JazzSong {
        name: String::from("CDF"),
    });
    let classic1 = Box::new(ClassicSong {
        category: String::from("UUUU"),
    });
    let classic2 = Box::new(ClassicSong {
        category: String::from("PPPPP"),
    });
    let songs: Vec<Box<dyn Playable>> = vec![jazz1, classic2, jazz2, classic1];
    for song in songs {
        song.play();
    }
}
