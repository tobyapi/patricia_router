# patricia_router
[![Build Status](https://travis-ci.org/TobiasGSmollett/patricia_router.svg?branch=master)](https://travis-ci.org/TobiasGSmollett/patricia_router) [![codecov](https://codecov.io/gh/TobiasGSmollett/patricia_router/branch/master/graph/badge.svg)](https://codecov.io/gh/TobiasGSmollett/patricia_router)  


Radix Tree implementation for Rust.

## Installation
Add this to your application's `Cargo.toml`.
```sh
[dependencies]
patricia_router = 0.1.0
```

## Usage
```rust
let mut router = Router::<&str>::new();
router.add("/", "root");
router.add("/*filepath", "all");
router.add("/products", "products");
router.add("/products/:id", "product");
router.add("/products/:id/edit", "edit");
router.add("/products/featured", "featured");

let mut result = router.find("/products/featured");
assert_eq!(result.key(), "/products/featured");
assert_eq!(result.payload, &Some("featured"));

// named parameters match a single path segment
result = router.find("/products/1000");
assert_eq!(result.key(), "/products/:id");
assert_eq!(result.payload, &Some("product"));

// catch all parameters match everything
result = router.find("/admin/articles");
assert_eq!(result.key(), "/*filepath");
assert_eq!(result.params("filepath"), "admin/articles");
```

## Development
Run tests following commands.
```sh
$ cargo test

$ rustup install nightly
$ cargo +nightly bench
```

Code submitted to this repository should be formatted according to `cargo +nightly fmt`.
```sh
$ rustup toolchain install nightly
$ cargo +nightly fmt
```

## Implementation
This project has been inspired and adapted from [luislavena/radix](https://github.com/luislavena/radix) Crystal implementation, respectively.
