A lossy image compression algorithm, based in a per-block level of detail detection, implemented in Rust.

# pixzlr

[![Crate downloads](https://img.shields.io/crates/d/pixlzr?logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Crate recent downloads](https://img.shields.io/crates/dr/pixlzr?logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Crate version](https://img.shields.io/crates/v/pixlzr?label=Version&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
![Last GitHub commit](https://img.shields.io/github/last-commit/guiga-zalu/pixlzr-rust?logo=github&style=flat-square)

## Installing

Just put `pixlzr = "0"` as one of the `[dependencies]` into your `Cargo.toml`.

## Using

```rust
use image::{open, DynamicImage};
use pixlzr::{process, tree_process, tree_full_process};

// ...

let image: DynamicImage = open("img.png").expect("Expected an image!");

process(&image, 64, Some(|v| v / 4.0))
    .save("img-processed.png")
    .expect("Error saving the processed image!");

tree_process(&image, 64, 0.25, Some(|v| v / 6.0))
    .save("img-processed-tree.png")
    .expect("Error saving the processed image!");

tree_full_process(&image, 0.25, Some(|v| v / 6.0))
    .save("img-processed-tree-full.png")
    .expect("Error saving the processed image!");
```

## Core concept

Yet to be written...  
Please check [GitHub:guiga-zalu/smart-pixelizer](https://github.com/guiga-zalu/smart-pixelizer), as it's the implementation in Node JS.
