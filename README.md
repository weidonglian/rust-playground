# rust-playground![status](https://github.com/weidonglian/rust-playground/workflows/rust-playground/badge.svg)

## learning keynotes from the C++ developer point of view

A collection of keynotes during my way of learning rust.

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

### for and interators

There are three common methods which can create iterators from a collection:

- iter(), which iterates over &T.
- iter_mut(), which iterates over &mut T.
- into_iter(), which iterates over T.

The `for in collection` will by default call the into_iter implicitly, thus will move `T` object. We need to explicitly call `iter()` and `iter_mut()` if needed.

```rust
let names = vec!["Bob", "Frank", "Ferris"];
for name in names.iter() {}
for name in names.iter_mut() {}
```
