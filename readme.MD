A lossy image compression algorithm, based in a per-block level of detail detection, implemented in Rust.

# pixzlr

[![Crate downloads](https://img.shields.io/crates/d/pixlzr?logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Crate recent downloads](https://img.shields.io/crates/dr/pixlzr?logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
[![Crate version](https://img.shields.io/crates/v/pixlzr?label=Version&logo=rust&style=for-the-badge)](https://crates.io/crates/pixlzr)
![Last GitHub commit](https://img.shields.io/github/last-commit/guiga-zalu/pixlzr-rust?logo=github&style=flat-square)

- [pixzlr](#pixzlr)
  - [Installing](#installing)
  - [Using](#using)
    - [Before using it](#before-using-it)
    - [Using the old API](#using-the-old-api)
    - [Using the new API](#using-the-new-api)
  - [CLI](#cli)
  - [Core concept](#core-concept)

## Installing

Just put `pixlzr = "0"` as one of the `[dependencies]` into your `Cargo.toml`.

## Using

### Before using it

Now, the `crate pixlzr` have two parallel APIs, with different uses and functionalities.

The base use, of reducing / shrinking a `crate image::DynamicImage`, is present in both. But in the future, the old API will be either deleted or rewritten based on the new one, which supports file saving / readding.

### Using the old API

```rust
use image::{open, DynamicImage};
use pixlzr::process::{
    process,
    tree::{
        process as tree_process,
        full as tree_full_process
    }
};

// ...

let image: DynamicImage = open("img.png")?;

process(&image, 64, Some(|v| v / 4.0)): DynamicImage
    .save("img-processed.png")?;

tree_process(&image, 64, 0.25, Some(|v| v / 6.0)): DynamicImage
    .save("img-processed-tree.png")?;

tree_full_process(&image, 0.25, Some(|v| v / 6.0)): DynamicImage
    .save("img-processed-tree-full.png")?;
```

### Using the new API

```rust
// Importing
use image::{open, DynamicImage};
use pixlzr::{FilterType, Pixlzr};
```

```rust
// Convert to
let png: DynamicImage = open("img.png");

let mut pix = Pixlzr::from_image(&img, 64, 64u32);
pix.shrink_by(FilterType::Gaussian, 0.8);

pix.save("pix-lized image.pixlzr")?;
```

```rust
// Convert from
let pix = Pixlzr::open("pix-lized image.pixlzr")?;
let img = pix.to_image(FilterType::Nearest)?;
img.save("reduced-img.png");
```

## CLI

The <abbr title="Command Line Interface">CLI</abbr> can be understood with a run of `pixlzr -h`.

```txt
Pixlzr - A rust lib and CLI for the pixlzr image format

Usage: pixlzr [OPTIONS] --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>                The input image file
  -o, --output <OUTPUT>              The output image file
  -b, --block-width <BLOCK_WIDTH>    The width of each block [default: 64]
      --block-height <BLOCK_HEIGHT>  The height of each block
  -k, --shrink-by <SHRINK_BY>        The shrinking factor [default: 1]
      --filter <FILTER>              The filter used when resizing the image blocks [default: lanczos3] [possible values: nearest, triangle, catmull-rom, gaussian, lanczos3]
  -h, --help                         Print help (see more with '--help')
  -V, --version                      Print version
```

It converts _from_ and _to_ the `pixlzr` format, with use of the `crate image`.

## Core concept

Yet to be written...  
Please check [GitHub:guiga-zalu/smart-pixelizer](https://github.com/guiga-zalu/smart-pixelizer), as it's the implementation in Node JS.
