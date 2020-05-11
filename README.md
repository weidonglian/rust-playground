# rust-playground![status](https://github.com/weidonglian/rust-playground/workflows/rust-playground/badge.svg)

## Learning keynotes

A collection of keynotes during my way of learning rust.

### Object lifetime

The 'static bound on a type doesn’t control how long that object lives; it controls the allowable lifetime of references that object holds. For example, String is a 'static bound object, since it holds no reference. If an object is bounded with 'static, it should hold no reference or a reference 'static bound.

```rust
fn test<T: Send + Sync + std::fmt::Display + 'static>(val: T) {
    thread::spawn(move || println!("{}", val));
}
```

The above code does not compile without 'static, since the compiler can not figure out the lifetime of type T.

One of the key things that a lot of people trip over is thinking that lifetime annotations refer to the lifetime of the object they are applied to. They do not; they refer to the minimum possible lifetime of any borrowed references that the object contains. This, of course, constrains the possible lifetimes of that object; an object cannot outlive its borrowed references, so its maximum possible lifetime must be shorter than the minimum possible lifetimes of the references it contains.

When handing an object off to a thread, it must have only 'static references, because the new thread could outlive the original thread.

[see more details](https://users.rust-lang.org/t/why-does-thread-spawn-need-static-lifetime-for-generic-bounds/4541).
