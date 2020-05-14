#[test]
fn variable_binding_and_expression_demo() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

#[test]
fn let_pre_declare() {
    let x;
    let y = 10;
    {
        x = y * y;
    }

    println!("x={}, y={}", x, y);
}

#[test]
fn let_shadow() {
    let x = 10;
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
