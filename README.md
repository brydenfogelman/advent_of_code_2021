# Rust Learnings

Here's some things I need to learn a bit better

## Day 1

## Day 2

### temporary variables

This works
```rust
let line = line.unwrap();
let line = line.split_whitespace().collect::<Vec<&str>>();
```

But this doesn't
```rust
let line = line.unwrap().split_whitespace().collect::<Vec<&str>>();
```

### reading from files 


## Day 3

wow ... this one was rough

### isize vs i32

???

### String vs &str

https://blog.thoughtram.io/string-vs-str-in-rust/

### lifetimes

???

### references

why did I need to add `&String` instead of `String`? Strings confuse the hell out of my in rust.
