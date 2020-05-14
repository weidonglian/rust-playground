# rust-playground![status](https://github.com/weidonglian/rust-playground/workflows/rust-playground/badge.svg)

## learning keynotes from the C++ developer point of view

A collection of keynotes during my way of learning rust.

### value vs reference sementic

The same as C++, rust also has the so-called value sementic `T` and reference/pointer sementic `T&`.

- The value `T` sementic will take over the object and you will have an independent one. If type implements `Copy` trait, then it will just copy the value in a sense of copying the memcpy, e.g. primitive types i32, f34; otherwise, the object will be moved (e.g. `String`) and you can not use it after being moved law enforced by rust compiler. If you still want to use it afterwards, then you have the option to clone the type if the type implements `Clone` which is more generic and can do anything you want in your implementation. Absolute, you have more advanced options as well, e.g. `Box`, `Rc`, `Arc`.
- The reference/pointer `T&` sementic is managed by the borrower checker to garantee the correctness of reference or pointer.

### object lifetime

The `'static` bound on a type doesn’t control how long that object lives; it controls the allowable lifetime of references that object holds. For example, `String` is a `'static` bound object, since it holds no reference. If an object is bounded with `'static`, it should hold no reference or a reference `'static` bound.

```rust
fn test<T: Send + Sync + std::fmt::Display + 'static>(val: T) {
    thread::spawn(move || println!("{}", val));
}
```

The above code does not compile without `'static`, since the compiler can not figure out the lifetime of type `T`.

One of the key things that a lot of people trip over is thinking that lifetime annotations refer to the lifetime of the object they are applied to. They do not; they refer to the minimum possible lifetime of any borrowed references that the object contains. This, of course, constrains the possible lifetimes of that object; an object cannot outlive its borrowed references, so its maximum possible lifetime must be shorter than the minimum possible lifetimes of the references it contains.

When handing an object off to a thread, it must have only `'static` references, because the new thread could outlive the original thread.

[see more details](https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541).

### for and interators

There are three common methods which can create iterators from a collection:

- iter(), which iterates over &T.
- iter_mut(), which iterates over &mut T.
- into_iter(), which iterates over T and move it into for loop and can not be used afterwards.

The `for in` will by default call the into_iter implicitly, thus will move `T` object. We need to explicitly call `iter()` and `iter_mut()` or with `&, mut&` on the collection if needed.

```rust
let names = vec!["Bob", "Frank", "Ferris"];
for name in names.iter() {} <==> for name in &names {}
for name in names.iter_mut() {} <==> for name in &mut names {}
```

### destructing in `let` or `match` binding

A `let` or `match` block can destructure items in a variety of ways:

- Destructuring Tuples
- Destructuring Enums
- Destructuring Pointers/Refs
- Destructuring Structures

### binding and expression

Left hand side is the binding and right hand side is the expression.

#### `ref` in the binding <==> `&` in the expression

```rust
let ref r = 2; // a reference to value 2
let r = &2; // a reference to value 2
```

#### `&` in the binding <==> `*` in the expression

```rust
let r = &1; // a reference
let v = *r; // v is the value
let &v = r; // v is the value
```

#### powerful `let if` and `while if`

```rust
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

// Create example variables
let a = Foo::Bar;
let b = Foo::Baz;
let c = Foo::Qux(100);

// Variable a matches Foo::Bar, work even if
// Enum Foo does not derive from `PartialEq`
if let Foo::Bar = a {
    println!("a is foobar");
}

// Variable b does not match Foo::Bar
// So this will print nothing
if let Foo::Bar = b {
    println!("b is foobar");
}

// Variable c matches Foo::Qux which has a value
// Similar to Some() in the previous example
if let Foo::Qux(value) = c {
    println!("c is {}", value);
}

// Binding also works with `if let`
if let Foo::Qux(value @ 100) = c {
    println!("c is one hundred");
}
```
