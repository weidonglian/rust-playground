#[test]
fn variable_binding_and_expression_demo() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = { 2 * x };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

#[test]
fn let_pre_declare() {
    let x;
    //println!("{}", x); compiler error
    let y = 10;
    {
        x = y * y;
    }

    println!("x={}, y={}", x, y);
}

#[test]
fn let_shadow() {
    let x = 10;
    println!("{}", x);
    let x = format!("{}", 10);
    println!("{}", x);
}

#[test]
fn let_by_block() {
    let mut x = {
        let y = 10;
        y * y
    };

    let y = loop {
        if x > 10 {
            x /= 2;
        } else {
            break x + 2;
        }
    };

    let z = if x == 10 { "A" } else { "B" };

    println!("[{}, {}, {}]", x, y, z);
}

#[test]
fn match_if() {
    let pair = (2, -2);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // The ^ `if condition` part is a guard
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

#[test]
fn match_binding() {
    // @ and if are equavalent
    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        Some(n) if n == 42 => panic!("should not reach here"),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }
}
