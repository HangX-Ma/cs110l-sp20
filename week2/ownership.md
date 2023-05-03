# Part 1: Ownership short-answer exercises

## Example 1

```rust
fn main() {
    let mut s = String::from("hello");
    let ref1 = &s;
    let ref2 = &ref1;
    let ref3 = &ref2;
    s = String::from("goodbye");
    println!("{}", ref3.to_uppercase());
}
```

The code can not compile. `s` is borrowed by `ref1` whose lifetime continues to the end of the program. Therefore, `s` can not be changed only if `ref1` gives back the ownership. If we want to use `s` afterwards, the easiest way is giving `ref1` a cloned `s`: `let ref1 = s.clone();`.

## Example 2

```rust
fn drip_drop() -> &String {
    let s = String::from("hello world!");
    return &s;
}
```

The code can not compile. The lifetime of `s` is limited within the `drip_drop` function. This variable will be dropped when the program goes out of scope. If we want to use this variable, we can directly return a copy.

- _Modified Code_

```rust
fn drip_drop() -> String {
    let s = String::from("hello world!");
    return s;
}
```

## Example 3

```rust
fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);
    let s2: String = v[0];
    println!("{}", s2);
}
```

The code can not compile. This [link](https://stackoverflow.com/questions/27876588/why-is-the-copy-trait-needed-for-default-struct-valued-array-initialization) talks about the `copy` and `clone` trait. By default, the primitive type value is implicitly copyable. Whereas in this example, it is incorrect or impossible to implement `Copy` for any type that contains a `String`. `String` can't implement `Copy` because it contains a pointer to some variable amount of heap memory. The only correct way to copy a `String` is to allocate a new block of heap memory to copy all the characters into, which is what `String`'s `Clone` implementation does.
