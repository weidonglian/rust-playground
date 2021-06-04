# Rust-Playground![status](https://github.com/weidonglian/rust-playground/workflows/rust-playground/badge.svg)

Some hand-pick learning keynotes from the C++ developer point of view.

A collection of keynotes during my way of learning rust.

## Semantic `value` vs `reference`

The same as C++, rust also has the so-called value semantic `T` and reference/pointer semantic `T&`.

- The value `T` semantic will take over the object and you will have an independent one. If type implements `Copy` trait,
then it will just copy the value in a sense of copying the `memcpy`, e.g. primitive types i32, f34; otherwise, the object
will be moved (e.g. `String`) and you can not use it after being moved law enforced by rust compiler. If you still want
to use it afterwards, then you have the option to clone the type if the type implements `Clone` which is more generic and
can do anything you want in your implementation. Absolute, you have more advanced options as well, e.g. `Box`, `Rc`, `Arc`.
- The reference/pointer `T&` semantic is managed by the borrower checker to guarantee the correctness of reference or pointer.

## Object lifetime

The `'static` bound on a type doesn’t control how long that object lives; it controls the allowable lifetime of references
that object holds. For example, `String` is a `'static` bound object, since it holds no reference. If an object is bounded
with `'static`, it should hold no reference or a reference `'static` bound.

```rust
fn test<T: Send + Sync + std::fmt::Display + 'static>(val: T) {
    thread::spawn(move || println!("{}", val));
}
```

The above code does not compile without `'static`, since the compiler can not figure out the lifetime of type `T`.

One of the key things that a lot of people trip over is thinking that lifetime annotations refer to the lifetime of the
object they are applied to. They do not; they refer to the minimum possible lifetime of any borrowed references that the
object contains. This, of course, constrains the possible lifetimes of that object; an object cannot outlive its borrowed
references, so its maximum possible lifetime must be shorter than the minimum possible lifetimes of the references it contains.

When handing an object off to a thread, it must have only `'static` references, because the new thread could outlive the
original thread.

[see more details](https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541).

## Looping with `for` and `iterators`

There are three common methods which can create iterators from a collection:

- iter(), which iterates over &T.
- iter_mut(), which iterates over &mut T.
- into_iter(), which iterates over T and move it into for loop and can not be used afterwards.

The `for in` will by default call the into_iter implicitly, thus will move `T` object. We need to explicitly call `iter()`
and `iter_mut()` or with `&, mut&` on the collection if needed.

```rust
let names = vec!["Bob", "Frank", "Ferris"];
for name in names.iter() {} <==> for name in &names {}
for name in names.iter_mut() {} <==> for name in &mut names {}
```

It is a common pattern in Rust to use function style to `transform` and then `collect`.

```rust
struct Shoe {
    pub size: u32,
    pub style: String,
}
let shoes: Vec<Shoe> = shoes.into_iter().filter(|x| x.size >= shoe_size).collect()
```

## Destructing in `let` or `match` binding

A `let` or `match` block can destructure items in a variety of ways:

- Destructuring Tuples
- Destructuring Enums
- Destructuring Pointers/Refs
- Destructuring Structures

## Binding & Expression

Left hand side is the binding and right hand side is the expression.

### `ref` in the binding <==> `&` in the expression

```rust
let ref r = 2; // a reference to value 2
let r = &2; // a reference to value 2
```

### `&` in the binding <==> `*` in the expression

```rust
let r = &1; // a reference
let v = *r; // v is the value
let &v = r; // v is the value
```

### powerful `let if` and `while if`

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

## Attributes or Annotations

- `#[cfg(test)]` indicates to be only built when `test` build configuration.
- `#[test]` tells the compiler this is a `test` function.
- `#[derive(PartialEq, Clone, Debug)]` will let the compiler provides the basic implementation for some derivable traits,
though you can still manually implement a more complex behavior if required. The following is a list of derivable traits:
  + Comparison traits: `Eq`, `PartialEq`, `Ord`, `PartialOrd`.
  + `Clone`, to create `T` from `&T` via a copy.
  + `Copy`, to give a type 'copy semantics' instead of 'move semantics'.
  + `Hash`, to compute a hash from &T.
  + `Default`, to create an empty instance of a data type.
  + `Debug`, to format a value using the `{:?}` formatter.
- `#[]`

## Closure `lambda`

use style `|i: i32| -> i32 { i + x }` in Rust <==> `[&x](int i) -> int { return i +x; }` in C++. However, the capturing
in Rust is inherently flexible and the compiler will try to make it work without annotations. It will try to capture by
`&T`, `&mut T`, finally `T`.
You could add `move` in the front, like `move || {}` to force capture by value, i.e. take the owner ship of the captured
variable.
