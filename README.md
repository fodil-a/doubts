Convenience, to write more explicit tests
=========================================

[![Build Status](https://travis-ci.org/rustyTheClone/doubts.svg?branch=master)](https://github.com/rustyTheClone/doubts)
[![Latest Version](https://img.shields.io/crates/v/doubts.svg)](https://crates.io/crates/doubts)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-orange.svg)](https://docs.rs/doubts/)

This crate provides an unique macro to write more explicit tests.

First, add dependency
```toml
[dev-dependencies]
doubts="0.1.0"
```
Then declare the use of this crate:
```rust
#[macro_use]
#[cfg(test)]
extern crate doubts;
```

Finally, write tests:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let v = vec![1, 2];
        assert_that!(v, has len == 2);
        assert_that!(v, has capacity == 2);
        assert_that!(v, contains &2);
    }
}
```